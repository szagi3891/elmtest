module Utils exposing (enumerate)

-- numeruje listę od 0 do length-1
enumerate : List a -> List (Int, a)
enumerate x = zip [0..(List.length x) - 1] x

zip : List a -> List b -> List (a,b)
zip = List.map2 (,)

