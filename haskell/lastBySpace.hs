#!/bin/env runhaskell

-- mucking around with haskell
--   reads text file from pipe and writes out the last field split by whitespace

import Data.Char  

dropEmpty :: [String] -> String
dropEmpty xs = 
    if filtered == [] then "" else ((reverse . head) filtered)
        where filtered = (dropWhile (\x -> x == []) xs)

lastField :: String -> [String] -> String
lastField [] soFar = dropEmpty soFar
lastField (x:xs) soFar = 
    if Data.Char.isSpace x 
        then
             if [] == (head soFar) 
                 then lastField xs soFar
                 else lastField xs ("" : soFar)
        else lastField xs ((x: head soFar): tail soFar)

lastFieldBySpace :: String -> String 
lastFieldBySpace line = lastField line [""] 
  
main = do  
    contents <- getContents  
    mapM_ putStrLn  (map lastFieldBySpace (lines contents))
