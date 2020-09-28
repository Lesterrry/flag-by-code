pub struct FlagByCode;
impl FlagByCode{
	pub fn flag(from: &str) -> &str{
		match from {
			"fr" => "🇫🇷",
			"en" => "🇬🇧",
			"ar" => "🇦🇪",
			"cs" => "🇨🇿",
			"da" => "🇩🇰",
			"de" => "🇩🇪",
			"el" => "🇬🇷",
			"es" => "🇪🇸",
			"fi" => "🇫🇮",
			"he" => "🇮🇱",
			"hi" | "bn" | "ta" => "🇮🇳",
			"it" => "🇮🇹",
			"jp" | "ja" => "🇯🇵",
			"ko" => "🇰🇷",
			"nl" => "🇧🇪",
			"hu" => "🇭🇺",
			"id" => "🇮🇩",
			"no" => "🇳🇴",
			"pl" => "🇵🇱",
			"pt" => "🇵🇹",
			"ro" => "🇷🇴",
			"ru" => "🇷🇺",
			"sk" => "🇸🇰",
			"sv" => "🇸🇪",
			"th" => "🇹🇭",
			"tr" => "🇹🇷",
			"zh" => "🇨🇳",

			"und" => "🏴",
			_ => "🏳️"
		}
	}
}
