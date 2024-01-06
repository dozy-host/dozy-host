trait Host {
    pub fn get_chat(&self) -> Option<Chat>;

    pub fn get_state(&self) -> HostState;
}