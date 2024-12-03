module Day_1 where

import System.IO

increases :: [Integer] -> Integer -> Integer
increases [x, y] n = if y > x then n+1 else n
increases (x:y:xs) n = if y > x then increases (y:xs) (n+1) else increases (y:xs) n

windowSum :: [Integer] -> [Integer]
windowSum [x,y,z] = [x + y + z]
windowSum (x:y:z:xs) = (x + y + z) : windowSum (y:z:xs)

main = do
    file <- openFile "../../input/2021/1.txt" ReadMode
    contents <- hGetContents' file
    let strnums = lines contents
    let nums = map readInt strnums
    print $ increases nums 0
    print $ increases (windowSum nums) 0


readInt :: String -> Integer
readInt = read
