import Data.Maybe
import Text.Read

main = do
    let exampleExpected = 8
        solver = possibleGamesSum

    example <- readFile "example.txt"
    let exampleOutput = solver example
    if exampleOutput /= exampleExpected
        then putStrLn $ "Incorrect example output: got " ++ show exampleOutput ++ ", expected " ++ show exampleExpected
        else do
            putStrLn $ "Correct example output: " ++ show exampleOutput
            input <- readFile "input.txt"
            putStr "Result: "
            print $ solver input

type CubeAmounts = (Int, Int, Int)

maxPossibleAmounts :: CubeAmounts
maxPossibleAmounts = (12, 13, 14)

possibleGamesSum :: (Num a) => String -> a
possibleGamesSum = fromIntegral . sum . mapMaybe possibleGameId . lines

possibleGameId :: String -> Maybe Int
possibleGameId s = gamePairToMaybe $ mapPair extractGameId areDrawsPossible $ takePair (splitOn ':' s)

extractGameId :: String -> Maybe Int
extractGameId = readMaybe . last . words

areDrawsPossible :: String -> Bool
areDrawsPossible = all isDrawPossible . splitOn ';' . tail

isDrawPossible :: String -> Bool
isDrawPossible = areAmountsPossible . parseAmounts

parseAmounts :: String -> CubeAmounts
parseAmounts s = foldl accAmounts (0, 0, 0) . map (takePair . words) $ splitOn ',' s

accAmounts :: CubeAmounts -> (String, String) -> CubeAmounts
accAmounts (r, g, b) (x, "red") = (r + read x, g, b)
accAmounts (r, g, b) (x, "green") = (r, g + read x, b)
accAmounts (r, g, b) (x, "blue") = (r, g, b + read x)

areAmountsPossible :: CubeAmounts -> Bool
areAmountsPossible (x, y, z) = (x <= r) && (y <= g) && (z <= b)
  where
    (r, g, b) = maxPossibleAmounts

gamePairToMaybe :: (Maybe a, Bool) -> Maybe a
gamePairToMaybe (Nothing, _) = Nothing
gamePairToMaybe (Just _, False) = Nothing
gamePairToMaybe (Just x, True) = Just x

mapPair :: (a -> c) -> (b -> d) -> (a, b) -> (c, d)
mapPair f g (x, y) = (f x, g y)

takePair :: [a] -> (a, a)
takePair (x : y : ys) = (x, y)

-- source: https://stackoverflow.com/a/7569301
-- copying this to avoid installing a package just to use Data.List.Split.splitOn
splitOn :: (Foldable t, Eq a) => a -> t a -> [[a]]
splitOn delimiter = foldr f [[]]
  where
    f c l@(x : xs)
        | c == delimiter = [] : l
        | otherwise = (c : x) : xs
