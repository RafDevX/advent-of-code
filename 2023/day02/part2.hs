main = do
  let exampleExpected = 2286
      solver = powerSum

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

powerSum :: (Num a) => String -> a
powerSum = fromIntegral . sum . map gamePower . lines

gamePower :: String -> Int
gamePower s = power . maxAmounts . map (parseAmounts . tail) . splitOn ';' . last $ splitOn ':' s

parseAmounts :: String -> CubeAmounts
parseAmounts s = foldl accAmounts (0, 0, 0) . map (takePair . words) $ splitOn ',' s

accAmounts :: CubeAmounts -> (String, String) -> CubeAmounts
accAmounts (r, g, b) (x, "red") = (r + read x, g, b)
accAmounts (r, g, b) (x, "green") = (r, g + read x, b)
accAmounts (r, g, b) (x, "blue") = (r, g, b + read x)

maxAmounts :: [CubeAmounts] -> CubeAmounts
maxAmounts [(r, g, b)] = (r, g, b)
maxAmounts ((r, g, b) : xs) = (max r x, max g y, max b z) where (x, y, z) = maxAmounts xs

power :: CubeAmounts -> Int
power (x, y, z) = x * y * z

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
