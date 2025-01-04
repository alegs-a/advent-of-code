module Day_11 where

import System.IO
import Data.List

inputPath = "../../input/2024/11.txt"

main = do
    file <- openFile inputPath ReadMode
    contents <- hGetContents' file
    let stones = readInt <$> words contents
    return ()

blink :: [a] -> [a]
blink (x:xs)
    | x == 0 = 1 : blink xs
    | even (logBase 10 x) = firsthalf x : xs

-- firsthalf :: (Integral a, Floating a) => a -> a
firsthalf x = div (mod (10 * div numDigits 2) x) (10 * div numDigits 2) where numDigits = digits x

-- digits :: (Num a, Floating a, RealFrac a) => a -> Integer
digits x = 1 + floor (logBase (fromIntegral x) 10)

readInt :: String -> Integer
readInt = read
