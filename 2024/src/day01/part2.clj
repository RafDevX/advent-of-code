(ns day01.part2
  (:require [clojure.test :refer [deftest]]
            [day01.part1 :as part1]
            [ysera.test :refer [is=]]))


(defn calc-score
  [list el]
  (* el
     (->> list
          (filter #(= % el))
          (count))))

(defn solve
  [name]
  (let [[as bs] (part1/historian-lists name)]
    (reduce + 0 (map (partial calc-score bs) as))))


(deftest example
  (is= (solve ::example)
       31))


(deftest input
  (is= (solve ::input)
       24316233))
