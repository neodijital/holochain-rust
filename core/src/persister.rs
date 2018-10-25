use holochain_core_types::error::HolochainError;
use state::State;
use std::{fs::{File,OpenOptions},sync::Arc,io::{Read,Write}};
use context::Context;

/// trait that defines the persistence functionality that holochain_core requires
pub trait Persister: Send {
    // @TODO how does save/load work with snowflake IDs?
    // snowflake is only unique across a single process, not a reboot save/load round trip
    // we'd need real UUIDs for persistant uniqueness
    // @see https://github.com/holochain/holochain-rust/issues/203
    fn save(&mut self, state: State)->Result<(),HolochainError>;
    fn load(&self,context:Arc<Context>) -> Result<Option<State>, HolochainError>;
}

#[derive(Default, Clone, PartialEq)]
pub struct SimplePersister {
    state: Option<State>,
    file_path : String
}

impl Persister for SimplePersister {
    fn save(&mut self, state: State)->Result<(),HolochainError> {
        let mut f = OpenOptions::new().write(true).create(true).open(self.file_path.clone())?;
        let json = State::serialize_state(state)?;
        Ok(f.write_all(json.as_bytes())?)
    }
    fn load(&self,context:Arc<Context>) -> Result<Option<State>, HolochainError> {
        let mut f = File::open(self.file_path.clone())?;
        let mut json = String::new();
        f.read_to_string(&mut json)?;
        let state = State::deserialize_state(context,json)?;
        Ok(Some(state))
    }
}

impl SimplePersister {
    pub fn new(file:String) -> Self {
        SimplePersister { state: None, file_path:file}
    }
}

#[cfg(test)]
mod tests {
    extern crate tempfile;
    use super::*;
    use self::tempfile::{tempdir,tempfile};
    use test_utils::create_test_context;
    use instance::tests::test_context;
    #[test]
    fn persistance_round_trip() 
    {
        let tempdir = tempdir().unwrap().path().join("test");
        let tempfile = tempdir.to_str().unwrap();
        let context = test_context("bob");
        File::create(tempfile.clone()).unwrap();
        let mut persistance  = SimplePersister::new(tempfile.to_string());
        let state = State::new(context.clone());
        persistance.save(state.clone()).unwrap();
        let state_from_file = persistance.load(context).unwrap().unwrap();
        assert_eq!(state,state_from_file )

    }

    // TODO write a persister.save() test
}
