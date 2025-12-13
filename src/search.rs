use scraper::{ElementRef, Html, Selector};
use crate::error::ShindenError;
use crate::models::{SearchPageResult, SearchRatings};
use crate::utils::extract_f64_after_colon;

fn extract_ratings(row: &ElementRef) -> Result<Option<SearchRatings>, ShindenError> {
    let rate_total_sel = Selector::parse(".rating-total span")
        .map_err(|e| ShindenError::HtmlError(e.to_string()))?;

    if row.select(&rate_total_sel).next().is_none() {
        return Ok(None);
    }

    let rate_story_sel = Selector::parse(".rating-story span")
        .map_err(|e| ShindenError::HtmlError(e.to_string()))?;
    let rate_graphics_sel = Selector::parse(".rating-graphics span")
        .map_err(|e| ShindenError::HtmlError(e.to_string()))?;
    let rate_music_sel = Selector::parse(".rating-music span")
        .map_err(|e| ShindenError::HtmlError(e.to_string()))?;
    let rate_chars_sel = Selector::parse(".rating-titlecahracters span")
        .map_err(|e| ShindenError::HtmlError(e.to_string()))?;


    Ok(Some(SearchRatings {
        overall: extract_f64_after_colon(row, &rate_total_sel),
        plot: extract_f64_after_colon(row, &rate_story_sel),
        graphics: extract_f64_after_colon(row, &rate_graphics_sel),
        music: extract_f64_after_colon(row, &rate_music_sel),
        characters: extract_f64_after_colon(row, &rate_chars_sel)
    }))
}