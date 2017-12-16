// Definite paths
let errorLog = './log/error.log'
let requestLog = './log/request.log'

// Requesting Logic
const requestJson = require( 'request-json' )
const request = require( 'request' )
const fs = require( 'fs' )


// The clients
var client = requestJson.createClient( 'https://boiling-cove-42309.herokuapp.com/' )
var testing = requestJson.createClient( 'http://132.162.201.225:3000/')
// The miners

// The passport
var passport = fs.readFileSync('./json/passport.json', 'utf8', function (err, data) {
    if (err) throw err;
})



// Post the passport to the client 
client.post( 'client/', JSON.parse( passport ), function( err, res, body )
             {
                 if ( err ) { throw err; }
                else if( res.statusCode == "450" ) { console.log( "Status code: " + res.statusCode ) }
                 else { console.log( res.statusCode + JSON.stringify( res.body ) ) }
                
            });

/*
client.post('remove/', JSON.stringify( passport ), function( err, res, body )
            {

                if ( err ) {
                    console.log("Error")
                }
                else if( res.statusCode == "450" ) { console.log( "Status code: " + res.statusCode)}
                else { console.log( res.body ) }
            });
   
*/

// Listening logic
const express = require( 'express' )
const server = express()

// Listen to get requests over the standard slug 
server.get( '/' , (req, res) => res.send( 'hello world!') )

server.post( '/register', ( req, res ) =>
             {
     
                 console.log( JSON.stringify( req.body ) )
                 res.status(260).json({"zac":"sucks"})                 
                 
             })

// Listen on port specified
server.listen( 3000, ( err ) =>

               {
                   if( err ) throw err;
                   console.log( 'Listening on 3000' )
               })

