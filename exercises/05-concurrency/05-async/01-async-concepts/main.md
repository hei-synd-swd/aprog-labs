ee# Questions: Async Concepts

Answer the questions below in your own words, in full sentences.

1. What does **calling** an `async fn` return, and when does its body run?
2. What is the difference between **preemptive** and **cooperative** scheduling?
3. Why must an `async fn` never call `std::thread::sleep` or run a long computation? What do you use instead?
4. Which part of the **runtime** polls the futures, and which handle tells it that a parked task can be polled again?
5. When do you choose **threads or Rayon**, and when **async with Tokio**?

<!-- Write your answer below -->
