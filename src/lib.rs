use zed_extension_api as zed;

struct RainbowBrackets;

impl zed::Extension for RainbowBrackets {
    fn new() -> RainbowBrackets {
        RainbowBrackets
    }

    // TODO
}

zed::register_extension!(RainbowBrackets);
