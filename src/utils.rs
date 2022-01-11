pub(crate) fn relative_quality_factor<T: ToString + ?Sized>(value: &T) -> Option<f32> {
    let value = value.to_string();
    if value.is_empty() {
        return None;
    };
    value
        .split(';')
        .nth(1)
        .and_then(|v| v.split("q=").nth(1))
        .and_then(|v| v.parse().ok())
        .or(Some(1.0f32))
}

pub(crate) fn array_from_string<T: ToString>(value: T) -> Vec<String> {
    value
        .to_string()
        .split(',')
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quality_10_1() {
        assert_eq!(relative_quality_factor("text/html"), Some(1.0f32));
    }

    #[test]
    fn quality_10_2() {
        assert_eq!(relative_quality_factor("text/html; q=1"), Some(1.0f32));
    }

    #[test]
    fn quality_10_3() {
        assert_eq!(relative_quality_factor("text/html; q=asd"), Some(1.0f32));
    }

    #[test]
    fn quality_07_1() {
        assert_eq!(relative_quality_factor("text/html; q=0.7"), Some(0.7f32));
    }

    #[test]
    fn quality_07_2() {
        assert_eq!(relative_quality_factor("text/html;q=0.7"), Some(0.7f32));
    }

    #[test]
    fn string2array_1() {
        assert!(array_from_string("").is_empty());
    }

    #[test]
    fn string2array_2() {
        assert_eq!(array_from_string("text/html;q=0.7").len(), 1);
    }

    #[test]
    fn string2array_3() {
        assert_eq!(array_from_string("text/html;q=0.7,,text/x-dvi").len(), 2);
    }

    #[test]
    fn string2array_4() {
        assert_eq!(
            array_from_string("text/html;q=0.7,,text/x-dvi"),
            vec!["text/html;q=0.7".to_string(), "text/x-dvi".to_string()]
        );
    }
}
