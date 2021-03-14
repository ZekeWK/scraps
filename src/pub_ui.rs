pub struct Database<'a> {
    file : &'a str
}

impl <'a> Database<'a> {
    pub fn lock_read(&self) -> Result<DatabaseGuardRead, Error> {
        todo!()
    }

    pub fn lock_write(&self) -> Result<DatabaseGuardWrite, Error> {
        todo!()
    }
}

pub struct DatabaseGuardRead<'a> {
    database : &'a Database<'a>
}

impl <'a> DatabaseGuardRead <'a> {
    pub fn get_entry(&self, index : u64, slice : &mut [u8] ) -> Result<(), Error> {
        self.database.get_entry(index, slice)
    }

    pub fn get_header(&self, slice : &mut [u8]) {
        self.database.get_header(slice)
    }

    pub fn len(&self) -> u64{
        self.database.len()
    }
}

impl <'a> Drop for DatabaseGuardRead<'a> {
    fn drop(&mut self) {
        todo!()
    }
}

pub struct DatabaseGuardWrite<'a> {
    database : &'a Database<'a>
}

impl <'a> DatabaseGuardWrite<'a> {
    pub fn get_entry(&self, index : u64, slice : &mut [u8] ) -> Result<(), Error> {
        self.database.get_entry(index, slice)
    }

    pub fn set_entry(&mut self, entry : &[u8], index : u64) -> Result<(), Error> {
        self.database_handler.set_entry(entry, index)
    }

    pub fn new_entry(&mut self, entry : &[u8]) -> Result<u64, Error> {
        self.database_handler.new_entry(entry)
    }

    pub fn pop(&mut self, slice : &mut [u8]) {
        self.database_handler.pop(slice)
    }

    pub fn get_header(&self, slice : &mut [u8]) {
        self.database_handler.get_header(slice)
    }

    pub fn len(&self) -> u64{
        self.database_handler.len()
    }
}


impl <'a> Drop for DatabaseGuardWrite <'a> {
    fn drop(&mut self) {
        todo!()
    }
}

pub enum Error {

}