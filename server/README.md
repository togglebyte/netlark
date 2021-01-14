Auth -> Load game state -> Overworld

## Game state
Game state holds:
* Map chunk
* Inventory
* Character stats

## Auth

General auth state -> 
    -> Sign in
    -> Sign up

* Sign in 
    * Username and password
* Sign up
    * Username and password and alternatively an email
* Sign out (this is not a real thing)

## Map / Tiles

* Sending tiles sends the X, Y and tile type to the client.

## Movement

* Server receives coords from player movement, and sends player pos update to
  all other players within the same space
 
## Combat

* Firing has a cooldown
* Player movement during combat will result in immediate movement by the target

Questions:
* How fast should the enemy unit move to get within attacking range of the
  player?
