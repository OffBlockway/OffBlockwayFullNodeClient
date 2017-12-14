// Definite paths
let errorLog = './log/error.log'
let requestLog = './log/request.log'

// Requesting Logic
const requestJson = require( 'request-json' )
const request = require( 'request' )

// The clients
var client = requestJson.createClient( 'https://boiling-cove-42309.herokuapp.com/' )
var testing = requestJson.createClient( 'https://132.162.201.225:3000/')
// The miners

// Post the passport to the client 
client.post( 'client/', { json: true }, './json/passport.json', function( err, res, body )
            {

                if ( err ) { console.log( "Status code: " + res.statusCode ) }
                else { console.log( res.statusCode ) }
                
            });

// Kosh post test
testing.post( 'test/', { json: true }, './json/passport.json', function( err, res, body)
              {
                  
                  if ( err ) { console.log( "Either not on ObieWifi or Kosh is not working: " + res.statusCode )}
                  else { console.log( "Kosh status: " + res.statusCode ) }
                      
              });

// Listening logic
const express = require( 'express' )
const server = express()

// Listen to get requests over the standard slug 
server.get( '/' , (req, res) => res.send( 'hello world!') )


// Listen on port specified
server.listen( 3000, () => console.log( 'Listening on 3000' ) )

