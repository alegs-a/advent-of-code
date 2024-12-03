module Day2 where

import System.IO

data Position = P Integer Integer

main = do
    file <- openFile "../../input/2021/1.txt" ReadMode
    contents <- hGetContents' file
    


readInt :: String -> Integer
readInt = read
