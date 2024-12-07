module Day_4 where

import System.IO
import Data.List

main = do
    file <- openFile "../../input/2024/4.txt" ReadMode
    contents <- hGetContents' file
    print $ sum $ map xmases $ chungus $ lines test
    print $ sum $ map xmases $ chungus $ lines test2
    print $ sum $ map xmases $ chungus $ lines contents

readInt :: String -> Integer
readInt = read

chungus :: [[a]] -> [[a]]
chungus x = concat $ allWays x ++ map diagonals (allWays x)

allWays :: [[a]] -> [[[a]]]
allWays x = [x, reversed, transposed, map reverse transposed]
    where
        transposed = transpose x
        reversed = map reverse x

xmases (x:m:a:s:xs) = if [x,m,a,s] == "XMAS" then 1 + xmases (m:a:s:xs) else xmases (m:a:s:xs)
xmases [m, a, s] = 0
xmases [a, s] = 0
xmases [s] = 0
xmases [] = 0

majorDiagonal :: [[a]] -> [a]
majorDiagonal (x:xs) = head x : majorDiagonal (map (drop 1) xs)
majorDiagonal [] = []

-- top-left to bottom-right diagonals
diagonals :: [[a]] -> [[a]]
diagonals [] = []
diagonals x = majorDiagonal x : upperDiagonals (upperRight x) ++ lowerDiagonals (lowerLeft x)

upperDiagonals [] = []
upperDiagonals x = majorDiagonal x : upperDiagonals (upperRight x)

lowerDiagonals [] = []
lowerDiagonals x = majorDiagonal x : lowerDiagonals (lowerLeft x)

upperRight :: [[a]] -> [[a]]
upperRight [] = []
upperRight (x:xs) = init $ map (drop 1) (x:xs)

lowerLeft :: [[a]] -> [[a]]
lowerLeft [] = []
lowerLeft (x:xs) = map init xs

thing = [[1,2,3], [4,5,6], [7,8,9]]

test = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX"
test2 = "XMAS\nXMAS\nXMAS\nXMAS"

