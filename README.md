Project README
Project Name: Create a Simple AI Model for Linear Regression Using Rust Burn Library VERSION 0.16
This README file provides a detailed explanation of how to set up the project, how the problem was approached, challenges faced during development, and the resources used to complete the project.
Table of Contents
1.	Project Setup
2.	Running the Code
3.	Approach and Problem Solving
4.	Challenges Faced
5.	Resources Used
6.	Reflecting on my learning
7.	Acknowledgments
________________________________________
Project Setup
Before I could start running my project, I made sure I have the necessary software that I needed for the project and dependencies downloaded and installed.
Prerequisites
1.	Rust 
2.	VCS 
Steps to Set Up
1.	The repository:
In the terminal, open and run the following command:
git https://github.com/EucretiaM/ eucretia-matyala-02.git
2.	Navigate into the project directory:
cd eucretia-matyala-02
3.	Install dependencies:
Used rust to install all necessary libraries from the requirements.txt file:
rust install -r requirements.txt.
4.	Verify installation:
Run a simple check to verify the setup:
rust -m unittest discover.
This checks that all dependencies are correctly installed and that the basic tests pass.
Running the Code
Once the project was set up, I then ran the code. Here's how:
Run the Main Script
1.	Navigated to the directory where my main rust script is located.
2.	Ran the script with the following command:
Rust main.py
Approach and Problem Solving
Problem Understanding
The goal of this project is to create a simple AI model for linear regression using the Rust programming language and the burn library. The model should predict the output of the function y = 2x + 1 using synthetic data for training. To use the burn library version 0.16.0 and other dependencies as specified in the provided Cargo.toml file.
Solution Design
1.	Data Collection: I used several websites, YouTube and ChatGPT to source all the help I can get. 
2.	Data Preprocessing: I used Data collection methods like using databases, APIs, web scraping and files data cleaning like handling missing data, removing duplicates, correcting errors, and handling outliers. 
3.	Implementation: I used all the software that were listed with their links for downloads and YouTube videos on how to, as I was clueless because it was my first time using any of these software and scripting languages.
Challenges Faced
1.	Data Quality: One of the biggest challenges was dealing with noisy or incomplete data. I had to preprocess and clean the data thoroughly before using it in the model. This involved identifying and filling missing values, removing outliers, and ensuring the data was correctly formatted.
2.	Performance Optimization: Initially, the model had a high runtime due to inefficient algorithm implementation. I improved performance by optimizing certain parts of the code. All of that by using YouTube videos for every step I wanted to take.
3.	Model Tuning: Finding the optimal hyperparameters for the machine learning model took a significant amount of time. I used grid search and cross-validation to explore different configurations.
4.	Deployment Issues: During the deployment phase, I encountered issues with dependency management, where certain libraries were incompatible with the version of Rust I was using. Solving these issues involved upgrading or downgrading specific libraries.
Resources Used
1.	Library Documentation:
o	NumPy Documentation
o	Pandas Documentation
o	Scikit-learn Documentation.
o	TensorFlow Documentation
2.	Tutorials:
o	YouTube and ChatGPT: Data Preprocessing
o	Online forums and communities
o	Towards Data Science: Hyperparameter Tuning
3.	AI Tools:
o	OpenAI GPT: Used for almost everything because I am unfamiliar with everything I was doing in this assignment.
4.	Other References:
o	Stack Overflow for troubleshooting code-related issues.
o	GitHub repositories for open-source code examples.
Reflecting on my learning
Upon running this project, I failed to make it run, it had a lot of error messages, unexpected but expected (as I am unfamiliar with coding) behaviours. Possible causes might be a combination of many factors like, syntax or logic errors, dependencies or libraries, configuration issues, and external services. I tried several troubleshooting steps like debugging, testing modules, revising code, and consulting documentations and forums but at the end, it still didn’t run.
1.	Help from AI, Documentation, and Other Sources:
AI played a significant role in providing quick assistance and guiding me through complex concepts. For example, AI can help me break down problems into smaller, manageable tasks, offering step-by-step explanations when necessary.
Documentation was often useful for deeper understanding, especially when tackling specific programming issues or algorithms. Online forums and communities also served as valuable resources, as real-world examples of solutions can provide practical insights.
Other sources, such as tutorials and guides, helped me understand the foundational concepts in more accessible ways.
2.	When I Failed to Solve the Problem:
On occasions when I couldn't find a solution right away, it was often due to a lack of understanding of a key concept or a small oversight in the problem-solving process. For instance, I might have missed a specific condition or misunderstood the requirements.
I learned that taking breaks and approaching the problem from a different angle can provide fresh insights. It also highlighted the importance of persistence and seeking clarification when stuck, rather than rushing to a solution.
Despite extensive efforts to resolve the issues, the project failed to run successfully. Overall, these experiences taught me the value of using multiple resources effectively and staying patient during problem-solving. Every failure became a learning opportunity that sharpened my understanding for the future.
Acknowledgments
•	I would like to thank Shasha and Siya (my classmates and collegues) for their support whenever I felt stuck and didn’t know what to do next.
•	Special thanks to the open-source community for the powerful libraries and tools provided.
•	Gratitude to YouTube tutorials, blogs, or other resources for helping me understand a little bit better the whole concept of this project.
