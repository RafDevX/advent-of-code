(ns util)

(defn get-resource-path
  [kw]
  {:pre ((qualified-keyword? kw))}
  (let [day (-> (namespace kw)
                (clojure.string/split #"\.")
                (first))
        resource (name kw)]
    (str "./src/" day "/" resource ".txt")))
