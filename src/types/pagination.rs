// use handle_errors::{AppError, ErrorType};
// use std::collections::HashMap;

/**
 * Pagination Struct responsible for storing {start} and {end} of the pagination request.
 */
#[derive(Debug)]
pub struct Pagination {
    /// The index of the first item.
    pub start: usize,
    /// The index of the last item.
    pub end: usize,
}

// pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, AppError> {
//     if params.contains_key("start") && params.contains_key("end") {
//         return Ok(Pagination {
//             start: params
//                 .get("start") // get returns an option because the key could not exist
//                 .unwrap() // unwrap actually returns the value of the Option (typically unsafe, but we check if the param exist beforehand)
//                 .parse::<usize>() // parse &str into a usize integer
//                 .map_err(Error::CannotParseParameter)?, // Returns a Result which we unwrap or an error
//             end: params
//                 .get("end")
//                 .unwrap()
//                 .parse::<usize>()
//                 .map_err(App::CannotParseParameter)?,
//         });
//     }

//     Err(Error::MissingParameters) // If  we couldn't catch a use case we throw a custom MissingParameters error
// }
