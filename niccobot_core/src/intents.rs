use serenity::model::gateway::GatewayIntents;

pub(crate) struct NiccoBotIntents {
    inner: GatewayIntents,
}

impl NiccoBotIntents {
    pub fn default() -> Self {
        Self {
            inner: GatewayIntents::empty()
                | GatewayIntents::GUILD_VOICE_STATES
                | GatewayIntents::GUILDS
                | GatewayIntents::GUILD_MESSAGES,
        }
    }

    pub fn inner(&self) -> GatewayIntents {
        self.inner
    }
}

impl From<NiccoBotIntents> for GatewayIntents {
    fn from(c_intents: NiccoBotIntents) -> Self {
        c_intents.inner()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_have_voice_intent() {
        let c_intents: GatewayIntents = NiccoBotIntents::default().into();
        assert!(c_intents.guild_voice_states());
    }

    #[test]
    fn should_have_guild_intents() {
        let c_intents: GatewayIntents = NiccoBotIntents::default().into();
        assert!(c_intents.guilds());
    }
}
