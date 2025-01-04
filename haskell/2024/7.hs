module Day_7 where

import System.IO
import Data.List

inputPath = "../../input/2024/7.txt"

parse :: String -> [(Integer, [Integer])]
parse contents = zip (readInt . takeWhile (/= ':') <$> lines contents) (map readInt . tail . words <$> lines contents)

main = do
    file <- openFile inputPath ReadMode
    contents <- hGetContents' file
    let eqns = parse contents
    print $ sum $ fst <$> filter isValid (parse testInput) -- test
    print $ sum $ fst <$> filter isValid eqns -- part 1
    print $ sum $ fst <$> filter isValid2 (parse testInput) -- test 
    print $ sum $ fst <$> filter isValid2 eqns -- part 2
    return ()

isValid (target, xs) = target `elem` allWays (reverse xs)

isValid2 (target, xs) = target `elem` allWays2 (reverse xs)

-- make sure to reverse input!
allWays [x, y] = [x + y, x * y]
allWays (x:xs) = ((+x) <$> rest) ++ ((*x) <$> rest)
    where rest = allWays xs

-- make sure to reverse input!
allWays2 [x, y] = [x + y, x * y, concat' y x]
allWays2 (x:xs) = ((+x) <$> rest) ++ ((*x) <$> rest) ++ (flip concat' x <$> rest) -- flip is necessary because we have to evaluate backwards for haskell to be happy
     where rest = allWays2 xs

readInt :: String -> Integer
readInt = read

concat' x y = readInt $ show x ++ show y

testInput = "190: 10 19\n3267: 81 40 27\n83: 17 5\n156: 15 6\n7290: 6 8 6 15\n161011: 16 10 13\n192: 17 8 14\n21037: 9 7 18 13\n292: 11 6 16 20"
a = [6, 8, 6, 15]
a' = reverse a
