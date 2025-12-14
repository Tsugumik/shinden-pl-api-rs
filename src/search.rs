use reqwest::Url;
use scraper::{ElementRef, Html, Selector};
use crate::client::ShindenHttpClient;
use crate::error::ShindenError;
use crate::headers::RequestType;
use crate::models::{SearchAnimeItem, SearchPageResult, SearchRatings};
use crate::utils::{extract_f64_after_colon, get_attr_from_selector, get_text_from_selector, get_u32_param_from_url, parse_f64_from_comma_str, parse_u32_from_str};

/// Searches for anime series on Shinden.pl.
///
/// # Arguments
///
/// * `client` - The `ShindenHttpClient` instance used to perform the request.
/// * `query` - The search phrase (e.g., "Hunter x Hunter").
/// * `page` - The page number to fetch (starts from 1).
/// # Returns
///
/// Returns a `Result` containing `SearchPageResult` with the list of anime and pagination info,
/// or a `ShindenError` if the request or parsing fails.
pub async fn search_anime(
    client: &ShindenHttpClient,
    query: &str,
    page: u32
) -> Result<SearchPageResult, ShindenError> {
    let mut url = Url::parse("https://shinden.pl/series")
        .map_err(|e| ShindenError::Config(e.to_string()))?;

    {
        let mut query_pairs = url.query_pairs_mut();
        query_pairs.append_pair("search", query);
        
        if page > 1 {
            query_pairs.append_pair("page", &page.to_string());
        }
    }

    let html = client.get_html(url.as_str(), RequestType::Frontend).await?;
    parse_search_html(&html)
}

/// Internal function to parse the raw HTML of the search results page.
fn parse_search_html(html: &str) -> Result<SearchPageResult, ShindenError> {
    let doc = Html::parse_document(html);

    let (current_page, total_pages) = extract_pagination(&doc)?;

    let anime_list = extract_anime_list(&doc)?;

    Ok(SearchPageResult {
        current_page,
        total_pages,
        anime_list
    })
}

fn extract_anime_list(doc: &Html) -> Result<Vec<SearchAnimeItem>, ShindenError> {
    let row_sel = Selector::parse(".div-row")
        .map_err(|e| ShindenError::HtmlParsing(e.to_string()))?;
    let mut list = Vec::new();

    for row in doc.select(&row_sel) {
        if let Ok(Some(item)) = parse_anime_row(&row) {
            list.push(item);
        }
    }

    Ok(list)
}

fn parse_anime_row(row: &ElementRef) -> Result<Option<SearchAnimeItem>, ShindenError> {
    let title_sel = Selector::parse(".desc-col h3 a")
        .map_err(|e| ShindenError::HtmlParsing(e.to_string()))?;
    let cover_sel = Selector::parse(".cover-col a")
        .map_err(|e| ShindenError::HtmlParsing(e.to_string()))?;
    let type_sel = Selector::parse(".title-kind-col")
        .map_err(|e| ShindenError::HtmlParsing(e.to_string()))?;
    let status_sel = Selector::parse(".title-status-col")
        .map_err(|e| ShindenError::HtmlParsing(e.to_string()))?;
    let episodes_sel = Selector::parse(".episodes-col")
        .map_err(|e| ShindenError::HtmlParsing(e.to_string()))?;
    let top_score_sel = Selector::parse(".rate-top")
        .map_err(|e| ShindenError::HtmlParsing(e.to_string()))?;
    let genres_sel = Selector::parse(".desc-col .tags li a")
        .map_err(|e| ShindenError::HtmlParsing(e.to_string()))?;

    let title = get_text_from_selector(row, &title_sel);

    if title.is_empty() {
        return Ok(None);
    }

    let url_suffix = get_attr_from_selector(row, &title_sel, "href");

    let full_url = if !url_suffix.is_empty() {
        format!("https://shinden.pl{}", url_suffix)
    } else {
        String::new()
    };

    let img_suffix = get_attr_from_selector(row, &cover_sel, "href");
    let image_url = if !img_suffix.is_empty() {
        format!("https://shinden.pl{}", img_suffix)
    } else {
        String::new()
    };

    let anime_type = get_text_from_selector(row, &type_sel);
    let status = get_text_from_selector(row, &status_sel);

    let ep_txt = get_text_from_selector(row, &episodes_sel);
    let episodes_count = parse_u32_from_str(&ep_txt);

    let score_txt = get_text_from_selector(row, &top_score_sel);
    let top_score = parse_f64_from_comma_str(&score_txt);

    let genres: Vec<String> = row.select(&genres_sel)
        .map(|el| el.text().collect::<String>().trim().to_string())
        .collect();

    let ratings = extract_ratings(row)?;

    Ok(Some(SearchAnimeItem {
        title,
        url: full_url,
        image_url,
        anime_type,
        episodes_count,
        status,
        top_score,
        search_ratings: ratings,
        genres
    }))
}

fn extract_ratings(row: &ElementRef) -> Result<Option<SearchRatings>, ShindenError> {
    let rate_total_sel = Selector::parse(".rating-total span")
        .map_err(|e| ShindenError::HtmlParsing(e.to_string()))?;

    if row.select(&rate_total_sel).next().is_none() {
        return Ok(None);
    }

    let rate_story_sel = Selector::parse(".rating-story span")
        .map_err(|e| ShindenError::HtmlParsing(e.to_string()))?;
    let rate_graphics_sel = Selector::parse(".rating-graphics span")
        .map_err(|e| ShindenError::HtmlParsing(e.to_string()))?;
    let rate_music_sel = Selector::parse(".rating-music span")
        .map_err(|e| ShindenError::HtmlParsing(e.to_string()))?;
    let rate_chars_sel = Selector::parse(".rating-titlecahracters span")
        .map_err(|e| ShindenError::HtmlParsing(e.to_string()))?;


    Ok(Some(SearchRatings {
        overall: extract_f64_after_colon(row, &rate_total_sel),
        plot: extract_f64_after_colon(row, &rate_story_sel),
        graphics: extract_f64_after_colon(row, &rate_graphics_sel),
        music: extract_f64_after_colon(row, &rate_music_sel),
        characters: extract_f64_after_colon(row, &rate_chars_sel)
    }))
}

fn extract_pagination(doc: &Html) -> Result<(u32, u32), ShindenError> {
    let pagination_sel = Selector::parse(".pagination")
        .map_err(|e| ShindenError::HtmlParsing(e.to_string()))?;

    let pagination_el = match doc.select(&pagination_sel).next() {
        Some(el) => el,
        None => return Ok((1, 1))
    };

    let current_page_sel = Selector::parse("li strong")
        .map_err(|e| ShindenError::HtmlParsing(e.to_string()))?;
    let last_page_sel = Selector::parse("li a[rel='last']")
        .map_err(|e| ShindenError::HtmlParsing(e.to_string()))?;

    let current_txt = get_text_from_selector(&pagination_el, &current_page_sel);
    let current_page = parse_u32_from_str(&current_txt).unwrap_or(1);

    let last_url_href = get_attr_from_selector(&pagination_el, &last_page_sel, "href");

    let total_pages = get_u32_param_from_url(&last_url_href, "page")
        .unwrap_or(current_page);

    Ok((current_page, total_pages))
}
