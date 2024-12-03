(ns day03.part2
  (:require [clojure.test :refer [deftest]]
            [util :refer [get-resource-path]]
            [ysera.test :refer [is=]]))


(defn process-input
  {:test (fn []
           (is= (process-input ::example2)
                [[2 4]
                 false
                 [5 5]
                 [11 8]
                 true
                 [8 5]]))}
  [name]
  (let [path (get-resource-path name)
        text (slurp path)]
    (->> (re-seq #"mul\((\d+),(\d+)\)|do\(\)|don't\(\)" text)
         (map (fn [m] (case (first m)
                        "do()" true
                        "don't()" false
                        (->> (rest m)
                             (map ^[String] Integer/parseInt)
                             (vec)))))
         (vec))))


(defn solve
  [name]
  (->> (process-input name)
       (reduce (fn [[acc active] instruction]
                 (cond
                   (boolean? instruction) [acc instruction]
                   active [(+ acc
                              (reduce * instruction))
                           active]
                   :else [acc active]))
               [0 true])
       (first)))


(deftest example
  (is= (solve ::example2)
       48))


(deftest input
  (is= (solve ::input)
       67269798))
