// Serde for JSON deserialization and serialization
extern crate serde;
extern crate serde_json;
extern crate off_blockway;
extern crate chrono;


// Used for timestamping 
use self::chrono::Utc;
use std::io;
use std::io::prelude::*;
use std::process::Command;
use self::serde_json::Value;
use std::io::{ Error, ErrorKind };
use std::vec::Vec;
use std::result::Result;
use off_blockway::*;
use self::transaction::Transaction;
use self::block::Block;
use self::chain::Chain;
use self::hash_util::*;
use self::merkle::Merkle;
use std::collections::hash_map::HashMap;
#[allow(unused_imports)]
use std::string::String;
// Used for writing to output files
#[allow(unused_imports)]
use std::fs::{ OpenOptions, File };
// Uses standard input / output
#[allow(unused_imports)]
use std::io::prelude::*;


// Registration information ( can be used for miner info )
#[derive( Clone, Serialize, Deserialize, Debug )]
pub struct Passport
{
    // The hash of the URL
    pub uid: String,
    // timestamp of current login
    pub timestamp: String,        
    // URL
    pub url: String
}

impl Passport
{

    // Constructor
    pub fn new() -> Passport
    {

        
        // Construct passport
        let mut url = Command::new( "ipconfig" ).args( &["getifaddr", "en0"] ).output().unwrap();
        let mut url_string: String = String::from_utf8( url.stdout ).unwrap();
        let len = url_string.clone().len();
        url_string.truncate( len - 2 );
        url_string = url_string.clone() + ":3000";
        
        // Construct timestamp
        let timestamp = Utc::now().to_string();

        // Construct uid
        let uid = create_leaf_hash( &url_string );

        Passport
        {

            uid: uid,
            timestamp: timestamp,
            url: url_string
            
        }
        
    }

    pub fn write_passport()
    {

        let passport = Passport::new();
        let mut file = File::create("json/passport.json").expect("Can not open file");
        file.write( serde_json::to_string( &Passport::new() ).unwrap().as_ref() );
        
    }

    
}


// The unit enums for the tuple index of each type
pub enum Parser
{

    UID,
    TRANSACTIONS,
    BLOCK
        
}

// Parser functions
impl Parser
{

    // Convert json to string
    pub fn to_json_from_file( filepath: &str ) -> Result< String, Error >
    {

        // Opens the file with the specified name
        let mut file = OpenOptions::new().read( true ).open( filepath );
        if file.is_err()
        {

            return Err( Error::new( ErrorKind::Other, "Could not open file" ) )
            
        }
        // Creates an emtpy string
        let mut json = String::new();
        // Reads the file as a string
        #[allow(unused_variables)]
        let temp = file.unwrap().read_to_string( &mut json );
        // Returns the String or Error
        Ok( ( json ) )
            
    }
    
    // Parse the package.json
    pub fn parse_package( filepath: &str ) -> Result< ( String, Vec<Transaction>, Block ), Error >
    {

        // Read in file and save if it exists
        let file_result = Parser::to_json_from_file( filepath );
        // Check if valid
        if file_result.is_err()
        {
            return Err( Error::new( ErrorKind::Other, "Could not open file" ) )
        }
        // The json value of the package 
        let value: Value = serde_json::from_str( &file_result? ).expect(" Can't get a value");       
        // Parse the values
        let block: Block = serde_json::from_str( &value["package"].to_string() ).expect("Can't parse the block");
        let transactions: Vec<Transaction> = serde_json::from_str( &value["transactions"]["nodes"].to_string() ).expect("Can't parse the transactions");
        let uid: String = serde_json::from_str( &value["uid"].to_string() ).expect("Can't parse UID");

        
        Ok( ( uid, transactions, block ) )
    }
    
    pub fn check_file( filepath: String ) -> bool
    {

        return true;
        
    }
    
}

// struct for checking block and merkle tree construction
pub struct Operator
{

    // Index
    pub index: usize,
    pub chain: Chain,
    pub merkle: HashMap< String, Merkle >
        
}


// Functions for the operator
impl Operator
{

    // Construct a new operator with the current information
    pub fn new( index: usize, chain: Chain, merkle: HashMap< String, Merkle > ) -> Operator
    {

        Operator
        {

            index: index,
            chain: chain,
            merkle: merkle
                
        }
        
    }
    // Create an empty operator
    pub fn empty( ) -> Operator
    {

        let mut hash_map = HashMap::new();
        hash_map.insert( empty_hash(), Merkle::empty() );
        
        
        Operator
        {

            index: 1, 
            chain: Chain::new(),
            merkle: hash_map     
        }
        
    }
    // Inject a new chain
    pub fn replace( &mut self, index: usize, chain: Chain, merkle: HashMap< String, Merkle > )
    {

        self.chain = chain;
        self.index = self.chain.len();
        self.merkle = merkle;
        
    }
    // push the block to the chain and build the merkle tree
    pub fn operate( &mut self, mut fields: ( String, Vec<Transaction>, Block ) )
    {

        fields.2.index = self.index.clone() as u64;
        self.index = ( self.index.clone() +  1 ) as usize;
        self.chain.push( fields.2.clone() );
        self.merkle.insert( fields.2.merkle_root().clone(), Merkle::new( fields.1 ) );
        
    }
    // 

}


