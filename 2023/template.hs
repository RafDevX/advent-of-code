main = do
    let exampleExpected = 123
        solver = length . lines

    example <- readFile "example.txt"
    let exampleOutput = solver example
    if exampleOutput /= exampleExpected
        then putStrLn $ "Incorrect example output: got " ++ show exampleOutput ++ ", expected " ++ show exampleExpected
        else do
            putStrLn $ "Correct example output: " ++ show exampleOutput
            input <- readFile "input.txt"
            putStr "Result: "
            print $ solver input
