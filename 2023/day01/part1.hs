import Data.Char
import Data.List
import Data.Maybe

main = do
    let exampleExpected = 142
        solver = calibrationSum

    example <- readFile "example.txt"
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
calibrationValue line = ((10 *) . firstDigit $ line) + lastDigit line

firstDigit :: String -> Int
firstDigit line = digitToInt . fromJust $ find isDigit line

lastDigit :: String -> Int
lastDigit = firstDigit . reverse
