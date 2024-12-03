(ns day03.part1
  (:require [clojure.test :refer [deftest]]
            [util :refer [get-resource-path]]
            [ysera.test :refer [is=]]))


(defn process-input
  {:test (fn []
           (is= (process-input ::example1)
                [[2 4]
                 [5 5]
                 [11 8]
                 [8 5]]))}
  [name]
  (let [path (get-resource-path name)
        text (slurp path)]
    (->> (re-seq #"mul\((\d+),(\d+)\)" text)
         (map rest)
         (map (partial map ^[String] Integer/parseInt))
         (map vec)
         (vec))))


(defn solve
  [name]
  (->> (process-input name)
       (map (partial reduce *))
       (reduce +)))


(deftest example
  (is= (solve ::example1)
       161))


(deftest input
  (is= (solve ::input)
       188741603))
