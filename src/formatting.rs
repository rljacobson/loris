/*!
Traits and structs for interfacing with and converting to strings. For the concrete atom string, see `atom/string.rs`.

*/

use std::borrow::Cow;

use strum::EnumString;


#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumString, Hash)]
pub enum DisplayForm {
  #[strum(serialize = "System`InputForm")]
  Input,
  #[strum(serialize = "System`FullForm")]
  Full,
  #[strum(serialize = "System`TraditionalForm")]
  Traditional,
  #[strum(serialize = "System`TeXForm")]
  TeX,
  #[strum(serialize = "System`StandardForm")]
  Standard,
  #[strum(serialize = "System`OutputForm")]
  Output,
}

impl Default for DisplayForm {
  fn default() -> DisplayForm {
    DisplayForm::Input
  }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, Default)]
/// Parameters used in methods that transform expressions into strings.
// Todo: This will probably need to include context at some point.
pub struct FormattingParameters {
  pub form            : DisplayForm,
  pub inside_top_level: bool
}

static DEFAULT_FORMATTING_PARAMETERS: Cow<FormattingParameters> = Cow::Owned(FormattingParameters{
      form: DisplayForm::Input,
      inside_top_level: false
    });

impl FormattingParameters {
  /**
    `FormattingParameters::standard()` differs from
    `FormattingParameters::default()` in that it returns a shared
    `Cow<FormattingParameters>` instead of `FormattingParameters`. Prefer
    this unless you know you will need mutability or to move the
    `FormattingParameters`.
  */
  pub fn standard() -> Cow<'static, FormattingParameters> {
    DEFAULT_FORMATTING_PARAMETERS.clone()
  }
}

impl From<DisplayForm> for FormattingParameters {
  fn from(form: DisplayForm) -> Self {
    FormattingParameters {
      form,
      inside_top_level: false
    }
  }
}
