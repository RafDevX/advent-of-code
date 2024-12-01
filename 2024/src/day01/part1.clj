(ns day01.part1
  (:require [clojure.test :refer [deftest]]
            [util :refer [get-resource-path]]
            [ysera.test :refer [is=]]))


(defn process-input
  {:test (fn []
           (is= (process-input ::example)
                [[3 4]
                 [4 3]
                 [2 5]
                 [1 3]
                 [3 9]
                 [3 3]]))}
  [name]
  (let [path (get-resource-path name)]
    (with-open [reader (clojure.java.io/reader path)]
      (->> (line-seq reader)
           (map #(clojure.string/split % #"\s+"))
           (map #(map ^[String] Integer/parseInt %))
           (map vec)
           (vec)))))


(defn historian-lists
  {:test (fn []
           (is= (historian-lists ::example)
                [[3 4 2 1 3 3]
                 [4 3 5 3 9 3]]))}
  [name]
  (let [input (process-input name)]
    (reduce (fn [[as bs] [a b]]
              [(conj as a)
               (conj bs b)])
            [[] []]
            input)))


(defn solve
  [name]
  (->> (historian-lists name)
       (map sort)
       (apply map vector)                                   ; Pairs
       (map (fn [[a b]] (abs (- a b))))                     ; Diffs
       (reduce +)))                                         ; Sum


(deftest example
  (is= (solve ::example)
       11))


(deftest input
  (is= (solve ::input)
       1666427))
