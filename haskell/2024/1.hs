module Day_1 where

import System.IO
import Data.List

main = do
    file <- openFile "../../input/2024/1.txt" ReadMode
    contents <- hGetContents' file
    let list = map tuplify . ((map readInt) . words) $ lines contents
    let (a, b) = unzip lists
    -- part 1
    let pairs = zip (sort a) (sort b)
    print $ sum $ map (abs . uncurry (-)) pairs
    -- part 2 
    print $ sum $ map (\x -> x * count b x) a

readInt :: String -> Integer
readInt = read

tuplify :: [a] -> (a,a)
tuplify [x, y] = (x,y)

count [] e = 0
count (x:xs) e = if e == x then 1 + count xs e else count xs e
