import Data.Char
import Data.List
import Data.Maybe

main = do
    let exampleExpected = 281
        solver = calibrationSum

    example <- readFile "example2.txt"
    let exampleOutput = solver example
    if exampleOutput /= exampleExpected
        then putStrLn $ "Incorrect example output: got " ++ show exampleOutput ++ ", expected " ++ show exampleExpected
        else do
            putStrLn $ "Correct example output: " ++ show exampleOutput
            input <- readFile "input.txt"
            putStr "Result: "
            print $ solver input

calibrationSum :: String -> Int
calibrationSum = sum . map calibrationValue . lines

calibrationValue :: String -> Int
calibrationValue line = ((10 *) . fromJust . firstDigit $ line) + (fromJust . lastDigit $ line)

spelledOutDigits :: [String]
spelledOutDigits = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]

firstDigit :: String -> Maybe Int
firstDigit line = firstDigit' line spelledOutDigits

firstDigit' :: String -> [String] -> Maybe Int
firstDigit' [] _ = Nothing
firstDigit' str@(x : xs) sod
    | isDigit x = Just $ digitToInt x
    | otherwise = case findIndex (`isPrefixOf` str) sod of
        Just i -> Just i
        Nothing -> firstDigit' xs sod

lastDigit :: String -> Maybe Int
lastDigit line = firstDigit' (reverse line) (map reverse spelledOutDigits)
