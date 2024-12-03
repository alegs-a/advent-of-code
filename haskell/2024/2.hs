module Day_2 where

import System.IO
import Data.List

main = do
    file <- openFile "../../input/2024/2.txt" ReadMode
    contents <- hGetContents' file
    let things = map (map readInt . words) (lines contents)
    print $ length $ filter isValid things
    print $ length $ filter isValid2 things
    return ()

isValid (x:y:xs) = isMonotone (x:y:xs) && (diff >= 1) && (diff <= 3) && isValid (y:xs)
    where diff = abs (x - y)
isValid [x] = True

allPerms x = [take (i - 1) x ++ drop i x | i <- [1..length x]]

isValid2 x = foldr ((||) . isValid) False (allPerms x)
isValid2' x = any $ map isValid $ allPerms x

readInt :: String -> Integer
readInt = read

isMonotone x = (x == sort x) || (x == reverse (sort x))

-- isValid2 (x:xs) = isValid2' (x:xs) || isValid xs
-- 
-- isValid2' (x:y:xs) = (isMonotone (x:y:xs) && (diff >= 1) && (diff <= 3) && isValid2' (y:xs)) || isValid (x:xs)
--     where sorted = sort (x:y:xs)
--           diff = abs (x - y)
-- isValid2' [x] = True
-- isValid2' [] = True
-- 

l1 = [7,6,4,2,1]
l2 = [1,2,7,8,9]
l3 = [9,7,6,2,1]
l4 = [1,3,2,4,5]
l5 = [8,6,4,4,1]
l6 = [1,3,6,7,9]
