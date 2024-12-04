module Day_3 where

import System.IO
import Data.List
import Control.Applicative
import qualified Control.Monad
import Control.Monad

newtype Parser a = P (String -> Maybe (a, String))

data Instruction = I Integer Integer
data Instructions = Is Instruction Instructions | Isf Instruction

instance Functor Parser where
    --fmap :: (a -> b) -> Parser a -> Parser b
    fmap g pa = do
      a <- pa
      return $ g a

instance Applicative Parser where
    --pure :: a -> Parser a
    pure a = P (\cs -> Just (a,cs))

    (<*>) :: Parser (a -> b) -> Parser a -> Parser b
    pg <*> pa = do
      g <- pg
      g <$> pa

instance Monad Parser where
    --(>>=) :: Parser a -> (a -> Parser b) -> Parser b
    p >>= f = P $ \cs ->
        case parse p cs of
          Nothing        -> Nothing
          Just (a, str') -> parse (f a) str'

instance Alternative Parser where
    empty = P $ const Nothing

    --(<|>) :: Parser a -> Parser a -> Parser a
    p <|> q = P $ \cs ->
        case parse p cs of
          Nothing -> parse q cs
          mx      -> mx

-- aux function for removing decorator
--parse :: Parser a -> String -> Maybe (a, String)
parse (P p) = p

-- parase one character
--item :: Parser Char
item = P foo
  where
    foo (c:cs) = Just (c, cs)
    foo _      = Nothing

-- parse a char c when P c.
--sat :: (Char -> Bool) -> Parser Char
sat p = do
    x <- item
    if p x then return x else empty

-- parse a digit
--digit :: Parser Char
digit = sat (\x -> elem x ['0'..'9'])

-- parse the character x
--char :: Char -> Parser Char
char x = sat (== x)
char' x = sat (/= x)

-- parse a natural number
nat :: Parser Integer
nat = read <$> some digit

mul = do
    char 'm'
    char 'u'
    char 'l'

dont = do
    char 'd'
    char 'o'
    char 'n'
    char '\''
    char 't'
    char '('
    char ')'
    <|>
    do
    item
    do'

do' = do
    char 'd'
    char 'o'
    char '('
    char ')'
    <|>
    do
    item
    do'

brackets = do
    char '('
    a <- nat
    char ','
    b <- nat
    char ')'
    pure (a, b)

junk x = void (many (char' x))

instruction = do
    mul
    (a, b) <- brackets
    pure $ I a b
    <|>
    do
    item
    instruction

instructions = do
    a <- instruction
    Is a <$> instructions
    <|>
    do
    Isf <$> instruction

instructions2 = do
    dont
    do'
    a <- instruction
    Is a <$> instructions
    <|>
    do
    a <- instruction
    Is a <$> instructions
    <|>
    do
    Isf <$> instruction


dum :: Maybe ([Instruction], String) -> [Instruction]
dum (Just ([], xs)) = case parse instruction xs of
    Just (i, rest) -> i : dum (Just ([], rest))
    nothing -> []
dum Nothing = []

string :: String -> Parser String
string = foldr (\ x -> (<*>) ((:) <$> char x)) (return [])

unpackh (I a b) = a * b

unpack (Is i is) = unpackh i : unpack is
unpack (Isf i) = [unpackh i]

eval (I a b) = a * b

main = do
    let testAnswer = parse instructions testInput
    case testAnswer of
        Just (p, r) -> print r
        Nothing -> print ":("
    print $ sum $ map eval $ dum $ Just ([], testInput)
    file <- openFile "../../input/2024/3.txt" ReadMode
    contents <- hGetContents' file
    let foo = parse instructions contents
    case foo of
        Just (p, _) -> print $ sum $ unpack p
    let testAnswer2 = parse instructions2 testInput2
    case testAnswer2 of
        Just (p, r) -> print r
        Nothing -> print ":("
    print $ sum $ map eval $ dum $ Just ([], testInput2)
    let foo = parse instructions2 contents
    case foo of
        Just (p, _) -> print $ sum $ unpack p
    return ()

testInput = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
testInput2 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"

--testInput = "%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5)) xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
-- testInput = ",5)+mul(32,64]then(mul(11,8)mul(8,5)) xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
-- testInput = "t_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5)) xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
-- testInput = "ul(32,64]then(mul(11,8)mul(8,5)) xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"

readInt :: String -> Integer
readInt = read

