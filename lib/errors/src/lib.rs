pub mod build_error;
pub mod check_error;
pub mod driver_error;
pub mod eval_error;
pub mod parse_error;
pub mod test_error;
pub mod web_error;

pub mod variants;

// Web errors
pub use variants::add_event_handler::AddEventHandler;
pub use variants::append_child::AppendChild;
pub use variants::could_not_cast::CouldNotCast;
pub use variants::create_element::CreateElement;
pub use variants::element_not_found::ElementNotFound;
pub use variants::get_attribute::GetAttribute;
pub use variants::set_attribute::SetAttribute;
pub use variants::trigger_event::TriggerEvent;
pub use variants::undefined_example::UndefinedExample;

// File system errors
pub use variants::dir_access::DirAccess;
pub use variants::file_access::FileAccess;

// Parsing
pub use variants::duplicate_definition::DuplicateDefinition;
pub use variants::missing_input::MissingInput;
pub use variants::remaining_input::RemainingInput;
pub use variants::undefined_main::UndefinedMain;
pub use variants::unexpected_rule::UnexpectedRule;
pub use variants::unknown_keyword::UnknownKeyword;

/// Checking
pub use variants::empty_case::EmptyCase;
pub use variants::free_type_variable::FreeTypeVariable;
pub use variants::kind_mismatch::KindMismatch;
pub use variants::name_mismatch::NameMismatch;
pub use variants::not_a_subtype::NotASubtype;
pub use variants::type_mismatch::TypeMismatch;

/// Evaluating
pub use variants::value_mismatch::ValueMismatch;

// Language Errors
pub use variants::no_kinding::NoKinding;
pub use variants::no_subtyping::NoSubtyping;
pub use variants::no_typing::NoTyping;
pub use variants::undefined_language::UndefinedLanguage;

/// Common errors
pub use variants::free_variable::FreeVariable;
pub use variants::index_out_of_bounds::IndexOutOfBounds;
pub use variants::undefined_label::UndefinedLabel;
pub use variants::undefined_location::UndefinedLocation;
pub use variants::unexpected_derivation::UnexpectedDerivation;

pub use variants::toml::Toml;
