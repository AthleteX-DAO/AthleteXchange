use std::cell::RefCell;
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::{
    program_error::ProgramError,
    //program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};
use std::borrow::BorrowMut;

pub struct Match<'a>{
    data: &'a RefCell<&'a mut [u8]>,
    offset: usize //what is this
}

impl<'a> Match<'a>{
    pub const LEN: usize = 32 + 32 + num_fights*2 + 2; //size of the structure in bytes

    fn slice(
        &self,
        data: &mut [u8]
    ) ->(
        &mut [u8;32], //Pubkey1
        &mut [u8;32], //pubkey2
        &mut [u8;num_fights], //P1 picks
        &mut [u8;num_fights], //P2 picks
        &mut [u8;2] //Their bet (what each of them entered not the total)
    ){
        mut_array_refs![
            array_mut_ref![data, self.offset, Match::LEN],
            32,
            32,
            num_fights,
            num_fights,
            2
        ]
    }

    pub fn get_player_1(&self) -> Result<Pubkey,ProgramError>{
        Ok(Pubkey::new_from_array(*self.slice(&mut self.data.borrow_mut()).0))
    }
    pub fn set_player_1(&self,value: Pubkey){
        self.slice(&mut self.data.borrow_mut()).0.copy_from_slice(value.as_ref());
    }

    pub fn get_player_2(&self) -> Result<Pubkey,ProgramError>{
        Ok(Pubkey::new_from_array(*self.slice(&mut self.data.borrow_mut()).1))
    }
    pub fn set_player_2(&self,value:Pubkey){
        self.slice(&mut self.data.borrow_mut()).1.copy_from_slice(value.as_ref());
    }

    pub fn update_p1_picks(&self,value:&[u8;num_fights]){
        self.slice(&mut self.data.borrow_mut()).2.copy_from_slice(value.as_ref());
    }

    pub fn update_p2_picks(&self,value:&[u8;num_fights]){
        self.slice(&mut self.data.borrow_mut()).3.copy_from_slice(value.as_ref());
    }

    /*
    pub fn get_bet(&self) -> Result<u16,ProgramError>{
        //should this be returned as a u16 or &[u8,2]
    }
    pub fn set_bet(&self,value: ){

    }
    */
    pub fn new(data: &'a RefCell<&'a mut [u8]>, offset: usize)-> Result<Match,ProgramError>{
        if data.borrow().len() < Self::LEN + offset {
            //Error
        }
        Ok(Match{data,offset})
    }
}