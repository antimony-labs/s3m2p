//! ═══════════════════════════════════════════════════════════════════════════════
//! FILE: lessons.rs | AI/src/lessons.rs
//! PURPOSE: AI/ML lesson definitions - structured from intuition to advanced
//! MODIFIED: 2025-01-02
//! LAYER: LEARN → AI
//! ═══════════════════════════════════════════════════════════════════════════════
//!
//! Curriculum designed for learners from beginners to advanced practitioners.
//! Each lesson starts with intuition and a demo, then builds to formal concepts.

/// Technical term that can have a popup explanation
#[derive(Clone)]
pub struct Term {
    pub word: &'static str,
    pub short: &'static str,  // One-line explanation
    pub detail: &'static str, // Full explanation for popup
}

/// Glossary of AI/ML technical terms used across lessons
pub static GLOSSARY: &[Term] = &[
    Term {
        word: "gradient descent",
        short: "Optimization algorithm that follows the slope downhill",
        detail: "An iterative optimization algorithm that adjusts parameters in the direction \
                 that most reduces the error. Like a blindfolded hiker feeling for the downhill \
                 direction to reach the valley.",
    },
    Term {
        word: "loss function",
        short: "Measures how wrong the model's predictions are",
        detail: "A mathematical function that quantifies the difference between predicted and \
                 actual values. Lower loss means better predictions. Also called cost function \
                 or objective function.",
    },
    Term {
        word: "learning rate",
        short: "Step size when updating parameters",
        detail: "Controls how much we adjust parameters in each training step. Too large and \
                 training becomes unstable; too small and training takes forever. Finding the \
                 right learning rate is crucial.",
    },
    Term {
        word: "epoch",
        short: "One complete pass through all training data",
        detail: "Training typically requires multiple epochs - repeatedly showing the model \
                 all examples. Each epoch refines the parameters further.",
    },
    Term {
        word: "batch",
        short: "Subset of data used in one training step",
        detail: "Instead of using all data at once (expensive) or one example (noisy), we use \
                 batches. Common sizes: 32, 64, 128. Batch size affects training speed and quality.",
    },
    Term {
        word: "neuron",
        short: "Basic computational unit in a neural network",
        detail: "Takes weighted inputs, sums them, applies an activation function, and outputs \
                 a value. Inspired by biological neurons but much simpler.",
    },
    Term {
        word: "activation function",
        short: "Non-linear function applied to neuron outputs",
        detail: "Adds the 'bends' that let networks learn complex patterns. Without activation \
                 functions, stacking layers would just be one big linear function. Common: ReLU, \
                 sigmoid, tanh.",
    },
    Term {
        word: "ReLU",
        short: "Rectified Linear Unit: max(0, x)",
        detail: "The most popular activation function. Simple and effective: outputs x if positive, \
                 0 if negative. Avoids vanishing gradients that plagued earlier networks.",
    },
    Term {
        word: "sigmoid",
        short: "S-shaped function squashing values to 0-1",
        detail: "Converts any number to a probability between 0 and 1. Used in logistic regression \
                 and output layers for binary classification.",
    },
    Term {
        word: "softmax",
        short: "Converts scores to probabilities that sum to 1",
        detail: "Used in multi-class classification outputs. Takes a vector of scores and outputs \
                 a probability distribution. The class with highest score gets highest probability.",
    },
    Term {
        word: "weight",
        short: "Learnable parameter that scales an input",
        detail: "Each connection between neurons has a weight. Training adjusts these weights to \
                 minimize error. The weights encode what the network has learned.",
    },
    Term {
        word: "bias",
        short: "Learnable offset added to weighted sum",
        detail: "Allows the neuron to shift its activation threshold. Like an intercept in linear \
                 regression. Every neuron typically has both weights and a bias.",
    },
    Term {
        word: "forward pass",
        short: "Computing outputs from inputs through the network",
        detail: "Data flows forward through layers: input → hidden layers → output. Each layer \
                 transforms the data using its weights, biases, and activation functions.",
    },
    Term {
        word: "backpropagation",
        short: "Algorithm for computing gradients in neural networks",
        detail: "Efficiently computes how each weight contributed to the error by propagating \
                 gradients backward through the network using the chain rule. The key algorithm \
                 that makes deep learning possible.",
    },
    Term {
        word: "overfitting",
        short: "Model memorizes training data instead of learning patterns",
        detail: "When a model performs great on training data but poorly on new data. Like a \
                 student who memorizes exam answers without understanding concepts. Combat with \
                 regularization, dropout, or more data.",
    },
    Term {
        word: "underfitting",
        short: "Model is too simple to capture patterns",
        detail: "When the model performs poorly on both training and test data. The model lacks \
                 capacity to represent the underlying patterns. Solution: use a more complex model.",
    },
    Term {
        word: "regularization",
        short: "Techniques to prevent overfitting",
        detail: "Methods like L1/L2 penalties, dropout, or early stopping that constrain the model \
                 to prefer simpler explanations. Helps the model generalize to new data.",
    },
    Term {
        word: "dropout",
        short: "Randomly ignore neurons during training",
        detail: "During each training step, randomly 'drop' some neurons (set output to 0). \
                 Forces the network to learn redundant representations, improving generalization.",
    },
    Term {
        word: "convolution",
        short: "Sliding filter operation that detects patterns",
        detail: "A small filter (e.g., 3×3) slides across the input, computing dot products. \
                 Detects local patterns like edges. The core operation in CNNs.",
    },
    Term {
        word: "pooling",
        short: "Downsampling operation in CNNs",
        detail: "Reduces spatial dimensions by taking max or average over regions. Provides \
                 translation invariance and reduces computation. Common: max pooling, average pooling.",
    },
    Term {
        word: "feature map",
        short: "Output of a convolutional layer",
        detail: "Each filter in a conv layer produces a feature map - a 2D array showing where \
                 that pattern was detected in the input. Deep layers detect increasingly abstract features.",
    },
    Term {
        word: "kernel",
        short: "The filter weights in a convolutional layer",
        detail: "Also called filter. A small matrix (e.g., 3×3) of learned weights that slides \
                 across the input. CNNs learn what these kernels should detect.",
    },
    Term {
        word: "filter",
        short: "Same as kernel - weights for convolution",
        detail: "The pattern detector in a convolutional layer. Early layers learn simple patterns \
                 (edges); deeper layers learn complex patterns (shapes, objects).",
    },
    Term {
        word: "attention",
        short: "Mechanism to focus on relevant parts of input",
        detail: "Learns which parts of the input to focus on for each output. Revolutionary \
                 mechanism behind transformers. Computes similarity between queries and keys to \
                 weight values.",
    },
    Term {
        word: "query",
        short: "In attention: what am I looking for?",
        detail: "One of three components of attention. The query vector represents 'what information \
                 do I need?' and is compared against all key vectors to determine relevance.",
    },
    Term {
        word: "key",
        short: "In attention: what information do I offer?",
        detail: "Paired with values. Keys are compared to queries to compute attention weights. \
                 High query-key similarity means that value is important for this output.",
    },
    Term {
        word: "value",
        short: "In attention: the actual information to retrieve",
        detail: "The content that gets mixed together based on attention weights. Values are \
                 weighted by attention scores and summed to produce output.",
    },
    Term {
        word: "transformer",
        short: "Architecture using attention as core mechanism",
        detail: "The architecture behind GPT, BERT, and modern AI. Replaces recurrence with \
                 attention, enabling parallelization and better long-range dependencies. \
                 'Attention is All You Need' (2017).",
    },
    Term {
        word: "embedding",
        short: "Dense vector representation of discrete items",
        detail: "Converts words/tokens into continuous vectors that capture meaning. Similar words \
                 have similar embeddings. Learned during training or pre-trained (Word2Vec, GloVe).",
    },
    Term {
        word: "tokenization",
        short: "Splitting text into processable units",
        detail: "Breaking text into tokens (words, subwords, or characters). Modern models use \
                 subword tokenization (BPE, WordPiece) to handle rare words and multiple languages.",
    },
    Term {
        word: "vocabulary",
        short: "Set of all tokens the model knows",
        detail: "The complete list of tokens the model can process. Typical size: 30k-50k tokens. \
                 Each token maps to an index, which maps to an embedding vector.",
    },
    Term {
        word: "reinforcement learning",
        short: "Learning from rewards rather than labels",
        detail: "Agent learns by trial and error, receiving rewards for good actions and penalties \
                 for bad ones. Like training a dog with treats. Used in games, robotics, and RLHF.",
    },
    Term {
        word: "reward",
        short: "Scalar signal indicating action quality",
        detail: "In RL, the environment provides rewards (+1 for good, -1 for bad, etc.). The \
                 agent's goal is to maximize cumulative reward over time.",
    },
    Term {
        word: "policy",
        short: "Strategy mapping states to actions",
        detail: "In RL, the policy defines what action to take in each state. Can be deterministic \
                 (always same action) or stochastic (probability distribution over actions).",
    },
    Term {
        word: "value function",
        short: "Expected future reward from a state",
        detail: "Estimates how good it is to be in a particular state. Helps the agent choose \
                 actions that lead to high-value states. Core concept in RL algorithms.",
    },
    Term {
        word: "generative model",
        short: "Model that creates new data samples",
        detail: "Learns the underlying distribution of data and can generate new examples. \
                 Includes GANs, VAEs, diffusion models. Contrast with discriminative models.",
    },
    Term {
        word: "discriminative model",
        short: "Model that classifies or predicts",
        detail: "Learns boundaries between classes. Answers 'given x, what is y?' Most supervised \
                 learning models are discriminative. Contrast with generative models.",
    },
    Term {
        word: "latent space",
        short: "Hidden representation space learned by model",
        detail: "Lower-dimensional space where similar inputs are close together. Allows \
                 interpolation between examples and semantic arithmetic. Core to VAEs and GANs.",
    },
];

/// A single AI/ML lesson
pub struct Lesson {
    pub id: usize,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub icon: &'static str,
    /// The hook - why should I care? (1-2 sentences)
    pub why_it_matters: &'static str,
    /// Intuitive explanation - no jargon (2-3 paragraphs + diagrams)
    pub intuition: &'static str,
    /// What the demo shows
    pub demo_explanation: &'static str,
    /// Key takeaways (what should stick)
    pub key_takeaways: &'static [&'static str],
    /// For those who want to go deeper
    pub going_deeper: &'static str,
    /// Mathematical notation (optional, hidden by default)
    pub math_details: &'static str,
    /// Implementation guide with code prompts
    pub implementation: &'static str,
}

/// All AI/ML lessons - ordered from intuition to advanced
pub static LESSONS: &[Lesson] = &[
    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 1: THE BIG PICTURE
    // ═══════════════════════════════════════════════════════════════════════════
    
    // LESSON 0: What is AI?
    Lesson {
        id: 0,
        title: "The Prediction Machine",
        subtitle: "What AI Actually Does (and Doesn't Do)",
        icon: "🧠",
        why_it_matters: "Before we build AI, we need to understand what it really is: a sophisticated pattern-matching system that learns from examples, not a sentient being.",
        
        intuition: r#"<h3>The Sorting Hat Analogy</h3>

Imagine you are the Sorting Hat from Harry Potter. You have seen thousands of students and know which ones went to each house. Now, a new student sits beneath you. You feel their courage (maybe 7/10), their ambition (6/10), their loyalty (9/10), their wit (5/10).

Based on patterns you have learned, you predict: <strong>Hufflepuff!</strong>

That is AI. It is not magic. It is not thinking. It is <strong>finding patterns in past data and applying them to new situations</strong>.

<strong>What AI Does:</strong>
• Pattern recognition (What does this look like?)
• Prediction (What comes next?)
• Classification (Which category?)
• Generation (What would fit here?)

<strong>What AI Does NOT Do:</strong>
• Understand meaning
• Have goals or desires
• Reason about abstract concepts
• Know when it is wrong

<strong>The Three Ingredients:</strong>

<div class="mermaid">
graph LR
    D[Data<br/>Examples to learn from] --> T[Training<br/>Finding patterns]
    M[Model<br/>Mathematical recipe] --> T
    T --> P[Predictions<br/>Applying patterns to new data]
</div>

All the math we will learn—neural networks, transformers, reinforcement learning—is just different ways to find and apply patterns."#,
        
        demo_explanation: r#"<strong>Try it yourself:</strong> The demo shows random shapes that you'll classify. Watch how your brain creates a "decision boundary" just like AI does.

<strong>Key insight:</strong> Your brain is doing the same computation as a neural network - finding features and drawing boundaries."#,
        
        key_takeaways: &[
            "AI learns patterns from examples, it does not reason like humans",
            "More data + better model = better predictions",
            "AI can be wrong, especially on data unlike its training set",
            "The goal is to minimize errors, not achieve perfection",
        ],
        
        going_deeper: r#"The field splits into: <strong>Supervised Learning</strong> (labeled examples like 'this is a cat'), <strong>Unsupervised Learning</strong> (finding structure in unlabeled data), and <strong>Reinforcement Learning</strong> (learning from rewards like a dog learning tricks). Modern 'generative AI' like ChatGPT uses all three techniques.

The Turing Test asked 'Can machines think?' but that's the wrong question. The right question is 'Can machines be useful?' And the answer is increasingly yes, even without 'true' understanding."#,
        
        math_details: r#"<p>At its core, AI is function approximation. We want to learn a function:</p>

$$f: X \rightarrow Y$$

<p>Given inputs $X$, predict outputs $Y$. Training finds parameters $\theta$ that minimize:</p>

$$\text{Loss}(\theta) = \sum_{i=1}^{n} L(f(x_i; \theta), y_i)$$

<p>Where $L$ is a loss function measuring prediction error. Common choices:</p>

<ul>
<li><strong>Mean Squared Error</strong> (regression): $L = (y - \hat{y})^2$</li>
<li><strong>Cross-Entropy</strong> (classification): $L = -y \log(\hat{y}) - (1-y)\log(1-\hat{y})$</li>
</ul>"#,
        
        implementation: r#"<h4>Building Your First AI Model</h4>

<pre><code>import numpy as np
from sklearn.linear_model import LogisticRegression

# Data: features and labels
X = np.array([[1, 2], [2, 3], [3, 1], [6, 5], [7, 8], [8, 7]])
y = np.array([0, 0, 0, 1, 1, 1])  # 0 = class A, 1 = class B

# Create and train model
model = LogisticRegression()
model.fit(X, y)

# Predict new example
new_point = [[5, 4]]
prediction = model.predict(new_point)
probability = model.predict_proba(new_point)

print(f"Predicted class: {prediction[0]}")
print(f"Probabilities: {probability[0]}")
</code></pre>

<h4>Prompt for Claude Code</h4>

<p><em>"Create a simple logistic regression classifier in Python using sklearn. Train it on the XOR problem and visualize the decision boundary with matplotlib."</em></p>"#,
    },

    // LESSON 1: The Learning Problem
    Lesson {
        id: 1,
        title: "The Learning Problem",
        subtitle: "Fitting Curves to Chaos",
        icon: "📈",
        why_it_matters: "Every AI system, from spam filters to self-driving cars, is fundamentally trying to solve the same problem: find a pattern in messy data.",
        
        intuition: r#"<h3>The Line of Best Fit</h3>

Imagine you are a coffee shop owner. You notice that when it is hotter outside, you sell more iced coffee. You have 100 days of data: temperature and sales.

You plot it. There is a pattern, but it is not perfect. Some days with the same temperature had different sales. Your job: <strong>draw a line that best captures the trend</strong>.

This is <strong>LINEAR REGRESSION</strong> - the simplest and most fundamental ML algorithm.

<h3>The Error Mountain</h3>

Imagine you are blindfolded on a mountainside. Your goal: reach the valley (lowest point). You can only feel the slope beneath your feet.

<strong>Strategy:</strong> Always step downhill.

This is <strong>GRADIENT DESCENT</strong>. The mountain is the "loss landscape." Each position is a choice of parameters. The valley is where predictions match reality best.

<div class="mermaid">
graph TD
    A[Random Starting Point] --> B{Calculate Slope}
    B --> C[Take Small Step Downhill]
    C --> D{At Valley?}
    D -->|No| B
    D -->|Yes| E[Found Best Parameters!]
</div>

<strong>Why it works:</strong> For most problems, if you keep going downhill, you eventually reach a valley. The learning rate controls step size - too big and you overshoot, too small and it takes forever."#,
        
        demo_explanation: r#"<strong>🎮 Try This:</strong>
1. Click on the canvas to add data points
2. Watch the line try to fit your data
3. Click "Train" to see gradient descent in action
4. Adjust the learning rate - what happens when it's too high? Too low?
5. Add noise to see how the fit changes

<strong>Key insight:</strong> The line moves toward the data, one small step at a time, guided by the gradient (slope)."#,
        
        key_takeaways: &[
            "Learning = finding parameters that minimize error",
            "Gradient descent: small steps downhill on the error surface",
            "More data generally helps, but outliers can hurt",
            "The simplest model that works is usually best (Occam's Razor)",
        ],
        
        going_deeper: r#"Linear regression has a closed-form solution (Normal Equation: $w = (X^TX)^{-1}X^Ty$), but gradient descent <strong>scales to billions of parameters</strong>. Modern deep learning is just gradient descent on enormous error landscapes with trillions of dimensions.

<strong>Variations:</strong>
• <strong>Batch Gradient Descent:</strong> Use all data per step (slow but stable)
• <strong>Stochastic GD:</strong> Use one example per step (fast but noisy)
• <strong>Mini-batch GD:</strong> Use small batches (best of both)
• <strong>Adam, RMSprop:</strong> Adaptive learning rates that adjust per parameter"#,
        
        math_details: r#"<p>Linear regression model:</p>

$$\hat{y} = wx + b$$

<p>Mean Squared Error loss:</p>

$$L = \frac{1}{n}\sum_{i=1}^{n}(y_i - (wx_i + b))^2$$

<p>Gradients (partial derivatives of loss w.r.t. parameters):</p>

$$\frac{\partial L}{\partial w} = \frac{2}{n}\sum_{i=1}^{n}(wx_i + b - y_i) \cdot x_i$$

$$\frac{\partial L}{\partial b} = \frac{2}{n}\sum_{i=1}^{n}(wx_i + b - y_i)$$

<p>Gradient descent update rule:</p>

$$w \leftarrow w - \alpha \frac{\partial L}{\partial w}$$
$$b \leftarrow b - \alpha \frac{\partial L}{\partial b}$$

<p>Where $\alpha$ is the learning rate (typically 0.01 to 0.1).</p>"#,
        
        implementation: r#"<h4>Gradient Descent from Scratch</h4>

<pre><code>import numpy as np
import matplotlib.pyplot as plt

# Generate data
np.random.seed(42)
X = np.random.rand(100) * 10
y = 2.5 * X + 1.5 + np.random.randn(100) * 2

# Initialize parameters
w, b = 0.0, 0.0
learning_rate = 0.01
n_iterations = 1000

# Training loop
for i in range(n_iterations):
    # Predictions
    y_pred = w * X + b
    
    # Compute gradients
    dw = (2/len(X)) * np.sum((y_pred - y) * X)
    db = (2/len(X)) * np.sum(y_pred - y)
    
    # Update parameters
    w -= learning_rate * dw
    b -= learning_rate * db
    
    # Track loss
    if i % 100 == 0:
        loss = np.mean((y_pred - y)**2)
        print(f"Iteration {i}: Loss = {loss:.4f}")

print(f"Final parameters: w = {w:.2f}, b = {b:.2f}")
</code></pre>

<h4>Prompt for Claude Code</h4>

<p><em>"Implement linear regression with gradient descent from scratch in Python. Visualize the loss curve and show how the fitted line improves over iterations."</em></p>"#,
    },

    // LESSON 2: Drawing Boundaries
    Lesson {
        id: 2,
        title: "Drawing Boundaries",
        subtitle: "When Lines Become Decisions",
        icon: "✂️",
        why_it_matters: "Most real AI problems are classification: Is this email spam? Is this tumor malignant? Will this customer churn? Learning to draw decision boundaries is the key.",
        
        intuition: r#"<h3>The Email Sorter</h3>

Imagine you are sorting mail. You need to separate spam from legitimate emails. You notice patterns:
• <strong>Spam</strong> has words like "FREE", "WINNER", "CLICK NOW"
• <strong>Legitimate</strong> emails mention people you know

In 2D, if you plot emails by two features (word count vs. exclamation marks), spam clusters in one region. Your job: <strong>draw a line separating them</strong>.

But life is not 2D. Real emails have thousands of features. You need a line in 1000-dimensional space. Same concept, harder to visualize.

<h3>The Sigmoid Squish</h3>

In regression, we predict continuous values. For classification, we need <strong>probabilities</strong> (0 to 1).

The <strong>sigmoid function</strong> squishes any number into the 0-1 range:
• Large positive numbers → close to 1
• Large negative numbers → close to 0
• Zero → 0.5

<div class="mermaid">
graph LR
    X[Features] --> W[Weighted Sum]
    W --> S[Sigmoid Function]
    S --> P[Probability 0-1]
</div>

<strong>Decision rule:</strong> If P ≥ 0.5, predict class 1; otherwise class 0."#,
        
        demo_explanation: r#"<strong>🎮 Try This:</strong>
1. Add points of class A (blue) and class B (orange)
2. Watch the decision boundary form in real-time
3. See the probability heatmap showing model confidence
4. Try overlapping classes - some errors are unavoidable
5. Observe how the boundary maximizes separation

<strong>Key insight:</strong> The model finds the line that minimizes misclassifications. The colored regions show probability - brighter means more confident."#,
        
        key_takeaways: &[
            "Classification finds boundaries between categories",
            "Sigmoid converts scores to probabilities (0 to 1)",
            "Decision boundary is where probability = 50%",
            "Overlapping classes mean some errors are unavoidable",
            "Logistic regression is linear - can only draw straight boundaries",
        ],
        
        going_deeper: r#"<strong>Logistic regression</strong> is the foundation of neural networks. Each neuron is essentially a logistic regression unit.

<strong>The XOR problem:</strong> Some datasets cannot be separated by a straight line. XOR (exclusive or) is the classic example:
• (0,0) → 0
• (0,1) → 1
• (1,0) → 1
• (1,1) → 0

No single line can separate these! This limitation led to the invention of <strong>multi-layer neural networks</strong>. By stacking layers, we can learn curved boundaries.

<strong>Multi-class classification:</strong> For more than 2 classes, we use <strong>softmax</strong> instead of sigmoid. It outputs a probability distribution over all classes."#,
        
        math_details: r#"<p>Logistic regression model:</p>

$$P(y=1|x) = \sigma(w^T x + b) = \frac{1}{1 + e^{-(w^T x + b)}}$$

<p>Cross-Entropy Loss (log loss):</p>

$$L = -\frac{1}{n}\sum_{i=1}^{n}\left[y_i \log(\hat{y}_i) + (1-y_i)\log(1-\hat{y}_i)\right]$$

<p>Why cross-entropy? It heavily penalizes confident wrong predictions. Being confidently wrong is worse than being uncertain.</p>

<p>Gradient of cross-entropy w.r.t. weights (remarkably simple!):</p>

$$\frac{\partial L}{\partial w} = \frac{1}{n}\sum_{i=1}^{n}(\hat{y}_i - y_i) x_i$$

<p>Multi-class softmax:</p>

$$P(y=k|x) = \frac{e^{w_k^T x}}{\sum_{j=1}^{K} e^{w_j^T x}}$$"#,
        
        implementation: r#"<h4>Logistic Regression Classifier</h4>

<pre><code>import numpy as np
import matplotlib.pyplot as plt
from sklearn.datasets import make_classification
from sklearn.linear_model import LogisticRegression

# Generate binary classification data
X, y = make_classification(n_samples=200, n_features=2, 
                           n_informative=2, n_redundant=0,
                           n_clusters_per_class=1, random_state=42)

# Train logistic regression
model = LogisticRegression()
model.fit(X, y)

# Visualize decision boundary
def plot_decision_boundary(model, X, y):
    h = 0.02  # Step size
    x_min, x_max = X[:, 0].min() - 1, X[:, 0].max() + 1
    y_min, y_max = X[:, 1].min() - 1, X[:, 1].max() + 1
    
    xx, yy = np.meshgrid(np.arange(x_min, x_max, h),
                        np.arange(y_min, y_max, h))
    
    Z = model.predict_proba(np.c_[xx.ravel(), yy.ravel()])[:, 1]
    Z = Z.reshape(xx.shape)
    
    plt.contourf(xx, yy, Z, alpha=0.3, cmap='RdYlBu')
    plt.scatter(X[:, 0], X[:, 1], c=y, cmap='RdYlBu', edgecolors='k')
    plt.title('Logistic Regression Decision Boundary')
    plt.show()

plot_decision_boundary(model, X, y)
</code></pre>

<h4>Prompt for Claude Code</h4>

<p><em>"Create a logistic regression classifier with sklearn. Generate a 2D dataset with make_classification and visualize the decision boundary with a probability heatmap."</em></p>"#,
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 2: NEURAL NETWORK FOUNDATIONS
    // ═══════════════════════════════════════════════════════════════════════════

    // LESSON 3: The Perceptron
    Lesson {
        id: 3,
        title: "The Artificial Neuron",
        subtitle: "Biology Inspires Math",
        icon: "⚡",
        why_it_matters: "The perceptron, invented in 1958, is where it all started. Understanding this simple model reveals the core mechanic that scales to trillion-parameter models.",

        intuition: r#"<h3>The Brain Analogy (Sort Of)</h3>

A biological neuron receives signals from other neurons. If the combined signal exceeds a threshold, it "fires" and sends its own signal.

The <strong>perceptron</strong> mimics this:
1. Receive inputs (features)
2. Multiply each by a weight (importance)
3. Sum them up
4. If sum exceeds threshold, output 1; else output 0

The weights encode what the neuron has "learned" is important.

<h3>The Voting Committee</h3>

Think of each input as a committee member voting on a decision. The weights are their influence. A weighted vote above the threshold means "yes."

<div class="mermaid">
graph LR
    X1[Input 1] -->|w₁| S((Weighted<br/>Sum))
    X2[Input 2] -->|w₂| S
    X3[Input 3] -->|w₃| S
    S --> A{≥ threshold?}
    A -->|Yes| Y1[Output 1]
    A -->|No| Y0[Output 0]
</div>

<h3>The XOR Problem</h3>

A single perceptron can learn AND, OR, and NOT. But it <strong>CANNOT</strong> learn XOR (exclusive or):
• (0,0) → 0
• (0,1) → 1
• (1,0) → 1
• (1,1) → 0

<strong>No single line can separate these!</strong> This limitation triggered the "AI Winter" of the 1970s. The solution: stack multiple perceptrons in layers."#,

        demo_explanation: r#"<strong>🎮 Try This:</strong>
1. Select a target function (AND, OR, XOR)
2. Watch the perceptron try to learn it
3. For AND/OR: Success! Decision boundary appears
4. For XOR: Failure! No line works
5. Toggle "Add Hidden Layer" to see XOR become solvable

<strong>Key insight:</strong> Linear models can only learn linearly separable problems. XOR proves we need depth."#,

        key_takeaways: &[
            "Perceptron: weighted sum + threshold decision",
            "Can learn linearly separable problems (AND, OR)",
            "Cannot learn XOR (famous limitation)",
            "Solution: multiple layers = neural networks",
            "Each weight represents learned importance of that input",
        ],

        going_deeper: r#"The <strong>perceptron learning rule</strong> (Rosenblatt, 1958) is guaranteed to converge for linearly separable data. But Minsky and Papert's 1969 book "Perceptrons" proved the XOR limitation, contributing to reduced AI funding for over a decade.

The solution (backpropagation through hidden layers) was discovered independently multiple times but didn't become widely known until Rumelhart, Hinton, and Williams' 1986 paper. This breakthrough ended the AI winter.

<strong>Modern relevance:</strong> Every neuron in a deep network is still fundamentally a perceptron, just with better activation functions (ReLU instead of step) and trained with backpropagation."#,

        math_details: r#"<p>Perceptron output:</p>

$$y = \begin{cases}
1 & \text{if } \sum_{i=1}^{n} w_i x_i + b > 0 \\
0 & \text{otherwise}
\end{cases}$$

<p>Or equivalently with step activation function:</p>

$$y = \text{step}(w^T x + b)$$

<p>Perceptron learning rule (for each misclassified example):</p>

$$w_i \leftarrow w_i + \alpha (y_{\text{true}} - y_{\text{pred}}) x_i$$

<p>Where $\alpha$ is the learning rate. This rule adjusts weights in the direction that reduces error.</p>

<p><strong>Why XOR fails:</strong> XOR is not linearly separable. Mathematically:</p>

$$\nexists w_1, w_2, b : \begin{cases}
w_1 \cdot 0 + w_2 \cdot 0 + b < 0 \\
w_1 \cdot 0 + w_2 \cdot 1 + b > 0 \\
w_1 \cdot 1 + w_2 \cdot 0 + b > 0 \\
w_1 \cdot 1 + w_2 \cdot 1 + b < 0
\end{cases}$$"#,

        implementation: r#"<h4>Perceptron from Scratch</h4>

<pre><code>import numpy as np

class Perceptron:
    def __init__(self, n_inputs, learning_rate=0.1):
        self.weights = np.zeros(n_inputs)
        self.bias = 0.0
        self.learning_rate = learning_rate

    def predict(self, x):
        activation = np.dot(self.weights, x) + self.bias
        return 1 if activation >= 0 else 0

    def train(self, X, y, epochs=10):
        for epoch in range(epochs):
            errors = 0
            for xi, target in zip(X, y):
                prediction = self.predict(xi)
                error = target - prediction

                if error != 0:
                    self.weights += self.learning_rate * error * xi
                    self.bias += self.learning_rate * error
                    errors += 1

            print(f"Epoch {epoch+1}: {errors} errors")
            if errors == 0:
                break

# Test on AND problem
X = np.array([[0, 0], [0, 1], [1, 0], [1, 1]])
y = np.array([0, 0, 0, 1])  # AND truth table

perceptron = Perceptron(n_inputs=2)
perceptron.train(X, y)

print("Testing AND:")
for xi, yi in zip(X, y):
    pred = perceptron.predict(xi)
    print(f"{xi} -> {pred} (expected {yi})")
</code></pre>"#,
    },

    // LESSON 4: Neural Networks
    Lesson {
        id: 4,
        title: "Layers of Abstraction",
        subtitle: "From Pixels to Concepts",
        icon: "🏗️",
        why_it_matters: "Neural networks solve XOR and much more. By stacking layers, they build hierarchies of features—the key insight that powers modern AI.",

        intuition: r#"<h3>The Feature Factory</h3>

Imagine recognizing a face:
• <strong>Layer 1:</strong> Detects edges (lines, curves)
• <strong>Layer 2:</strong> Combines edges into parts (eyes, nose, mouth)
• <strong>Layer 3:</strong> Combines parts into faces
• <strong>Layer 4:</strong> Recognizes specific people

Each layer builds more abstract concepts from simpler ones. You don't program these features—the network <strong>discovers</strong> them from data!

<h3>The Information Bottleneck</h3>

As information flows through layers, it gets compressed and refined. Irrelevant details are discarded; relevant patterns are amplified.

<div class="mermaid">
graph LR
    I[Input<br/>1M pixels] --> H1[Hidden 1<br/>10K features]
    H1 --> H2[Hidden 2<br/>1K features]
    H2 --> H3[Hidden 3<br/>100 concepts]
    H3 --> O[Output<br/>10 classes]
</div>

<h3>Activation Functions: The Non-Linearity</h3>

Without activation functions, stacking layers would just be one big linear function. Activations (like <strong>ReLU: max(0, x)</strong>) add the "bends" that let networks learn curves.

<strong>Why ReLU?</strong>
• Simple: just max(0, x)
• Fast to compute
• Avoids vanishing gradients
• Empirically works great"#,

        demo_explanation: r#"<strong>🎮 Try This:</strong>
1. Start with the Spiral dataset (impossible for linear model)
2. Add hidden layers and neurons
3. Watch the decision boundary transform
4. Compare activation functions: ReLU vs Sigmoid vs Tanh
5. Observe how deeper networks learn more complex boundaries

<strong>Key insight:</strong> More layers = more abstraction capacity. The network learns to represent complex functions through composition of simple transformations."#,

        key_takeaways: &[
            "Layers build hierarchical features (edges → shapes → objects)",
            "Activation functions enable non-linear patterns",
            "More layers = more abstraction capacity",
            "Backpropagation trains all layers simultaneously",
            "Deep networks are exponentially more efficient than wide networks",
        ],

        going_deeper: r#"The <strong>Universal Approximation Theorem</strong> (Cybenko, 1989) proves that a network with one hidden layer can approximate any continuous function. But deeper networks are <strong>exponentially more efficient</strong>—they can represent the same functions with far fewer neurons.

<strong>Example:</strong> Computing parity (is the number of 1s even?) requires exponentially many neurons in a single hidden layer, but only linear growth with depth.

<strong>Depth vs Width trade-off:</strong>
• Shallow networks: Need exponentially many neurons
• Deep networks: Compositional structure matches problem structure
• Modern practice: Go deep (10-1000+ layers) with moderate width"#,

        math_details: r#"<p>Forward pass through layer $\ell$:</p>

$$a^{(\ell)} = \sigma(W^{(\ell)} a^{(\ell-1)} + b^{(\ell)})$$

<p>Where:</p>
<ul>
<li>$a^{(\ell)}$ = activations (outputs) of layer $\ell$</li>
<li>$W^{(\ell)}$ = weight matrix for layer $\ell$</li>
<li>$b^{(\ell)}$ = bias vector for layer $\ell$</li>
<li>$\sigma$ = activation function (ReLU, sigmoid, tanh)</li>
</ul>

<p>Common activation functions:</p>

$$\text{ReLU}(x) = \max(0, x)$$
$$\text{Sigmoid}(x) = \frac{1}{1 + e^{-x}}$$
$$\text{Tanh}(x) = \frac{e^x - e^{-x}}{e^x + e^{-x}}$$

<p>Universal Approximation: For any continuous function $f$ and $\epsilon > 0$, there exists a neural network $g$ such that:</p>

$$|f(x) - g(x)| < \epsilon \quad \forall x$$"#,

        implementation: r#"<h4>Multi-Layer Neural Network</h4>

<pre><code>import numpy as np

def relu(x):
    return np.maximum(0, x)

def relu_derivative(x):
    return (x > 0).astype(float)

class NeuralNetwork:
    def __init__(self, layers):
        self.weights = []
        self.biases = []

        # Initialize weights and biases
        for i in range(len(layers) - 1):
            w = np.random.randn(layers[i], layers[i+1]) * 0.1
            b = np.zeros((1, layers[i+1]))
            self.weights.append(w)
            self.biases.append(b)

    def forward(self, X):
        self.activations = [X]
        self.z_values = []

        for w, b in zip(self.weights, self.biases):
            z = self.activations[-1] @ w + b
            a = relu(z)
            self.z_values.append(z)
            self.activations.append(a)

        return self.activations[-1]

    def backward(self, X, y, learning_rate=0.01):
        m = X.shape[0]

        # Output layer gradient
        delta = self.activations[-1] - y

        # Backpropagate through layers
        for i in range(len(self.weights) - 1, -1, -1):
            dw = self.activations[i].T @ delta / m
            db = np.sum(delta, axis=0, keepdims=True) / m

            self.weights[i] -= learning_rate * dw
            self.biases[i] -= learning_rate * db

            if i > 0:
                delta = (delta @ self.weights[i].T) * relu_derivative(self.z_values[i-1])

# Create network: 2 inputs, two hidden layers (8, 8), 1 output
nn = NeuralNetwork([2, 8, 8, 1])

# Train on XOR
X = np.array([[0, 0], [0, 1], [1, 0], [1, 1]])
y = np.array([[0], [1], [1], [0]])

for epoch in range(1000):
    output = nn.forward(X)
    nn.backward(X, y, learning_rate=0.5)

    if epoch % 100 == 0:
        loss = np.mean((output - y)**2)
        print(f"Epoch {epoch}: Loss = {loss:.4f}")
</code></pre>"#,
    },

    // LESSON 5: Backpropagation
    Lesson {
        id: 5,
        title: "The Credit Assignment Problem",
        subtitle: "How Networks Learn From Mistakes",
        icon: "🔄",
        why_it_matters: "Backpropagation is the algorithm that makes deep learning possible. It efficiently computes how to adjust millions of weights to reduce errors.",

        intuition: r#"<h3>The Blame Game</h3>

You order a pizza. It arrives burnt. Who is to blame?
• The oven? (Too hot)
• The cook? (Left it in too long)
• The order taker? (Wrote "extra crispy")

<strong>Credit assignment:</strong> figuring out which decision caused the outcome.

In neural networks:
• Output is wrong
• Which weights caused the error?
• How should each weight change?

<h3>The Chain Rule: Responsibility Flows Backward</h3>

If A affects B, and B affects C, then A affects C <strong>through</strong> B.

Error at output → flows back through Layer 3 → Layer 2 → Layer 1 → Input weights

Each weight learns: <strong>"How much did I contribute to the error?"</strong>

<div class="mermaid">
graph RL
    E[Error] --> |∂L/∂a₃| H3[Layer 3]
    H3 --> |∂L/∂a₂| H2[Layer 2]
    H2 --> |∂L/∂a₁| H1[Layer 1]
    H1 --> |∂L/∂w| W[Weight<br/>Updates]
</div>

<h3>The Micrograd Insight</h3>

Every operation in a neural network can be decomposed into simple steps (add, multiply, activation). Each step has a <strong>known derivative</strong>. Chain them together = backpropagation!"#,

        demo_explanation: r#"<strong>🎮 Try This:</strong>
1. Watch the forward pass: values flow left to right
2. See loss computation at the end
3. Observe backward pass: gradients flow right to left
4. Color intensity shows gradient magnitude
5. Click nodes to see gradient calculations

<strong>Key insight:</strong> Gradients tell each weight "move this direction to reduce error." Larger gradients = stronger influence on the error."#,

        key_takeaways: &[
            "Backprop uses the chain rule to compute gradients",
            "Gradients flow backward from loss to inputs",
            "Each weight learns how much it contributed to error",
            "Vanishing gradients plague deep networks (solved by ReLU, skip connections)",
            "Automatic differentiation makes this all automatic in modern frameworks",
        ],

        going_deeper: r#"<strong>Automatic differentiation</strong> libraries (PyTorch, TensorFlow) build computational graphs automatically. You define the forward pass; gradients are computed for free. This is why modern ML code looks like normal Python—the magic is hidden.

<strong>Vanishing gradients:</strong> In deep networks with sigmoid/tanh, gradients get exponentially smaller in early layers. Gradients "vanish" and early layers don't learn. Solutions:
• <strong>ReLU activation:</strong> Gradient is either 0 or 1, no shrinkage
• <strong>Batch normalization:</strong> Keeps activations in a good range
• <strong>Residual connections:</strong> Skip paths let gradients flow directly

<strong>The bigger picture:</strong> Backprop is just gradient descent applied to neural networks. The network is the function, weights are parameters, and we're finding the parameter values that minimize loss."#,

        math_details: r#"<p>Chain rule for composition of functions:</p>

$$\frac{\partial L}{\partial x} = \frac{\partial L}{\partial y} \cdot \frac{\partial y}{\partial x}$$

<p>For a network with layers $f_1, f_2, \ldots, f_n$:</p>

$$\frac{\partial L}{\partial \theta_i} = \frac{\partial L}{\partial f_n} \cdot \frac{\partial f_n}{\partial f_{n-1}} \cdots \frac{\partial f_{i+1}}{\partial f_i} \cdot \frac{\partial f_i}{\partial \theta_i}$$

<p>Backpropagation efficiently computes this by caching intermediate values during forward pass.</p>

<p>For layer $\ell$:</p>

$$\delta^{(\ell)} = \left(W^{(\ell+1)}\right)^T \delta^{(\ell+1)} \odot \sigma'(z^{(\ell)})$$

<p>Where $\delta^{(\ell)} = \frac{\partial L}{\partial z^{(\ell)}}$ and $\odot$ is element-wise multiplication.</p>

<p>Weight gradient:</p>

$$\frac{\partial L}{\partial W^{(\ell)}} = \delta^{(\ell)} \left(a^{(\ell-1)}\right)^T$$"#,

        implementation: r#"<h4>Computational Graph with Autograd</h4>

<pre><code>import torch
import torch.nn as nn

# PyTorch does automatic differentiation
x = torch.tensor([[1.0, 2.0]], requires_grad=True)
w1 = torch.tensor([[0.5, 0.3], [0.2, 0.4]], requires_grad=True)
w2 = torch.tensor([[0.1], [0.6]], requires_grad=True)

# Forward pass (PyTorch builds computation graph automatically)
h = torch.relu(x @ w1)  # Hidden layer
y = h @ w2              # Output
loss = (y - 1.0) ** 2   # Loss

# Backward pass (computes all gradients automatically!)
loss.backward()

print("Gradient of loss w.r.t. w1:", w1.grad)
print("Gradient of loss w.r.t. w2:", w2.grad)
print("Gradient of loss w.r.t. x:", x.grad)

# The framework computed the chain rule for us!
</code></pre>

<h4>Manual Backprop (Educational)</h4>

<pre><code>class Value:
    """Micrograd-style autograd value"""
    def __init__(self, data, _children=(), _op=''):
        self.data = data
        self.grad = 0.0
        self._backward = lambda: None
        self._prev = set(_children)
        self._op = _op

    def __add__(self, other):
        out = Value(self.data + other.data, (self, other), '+')

        def _backward():
            self.grad += out.grad
            other.grad += out.grad
        out._backward = _backward
        return out

    def __mul__(self, other):
        out = Value(self.data * other.data, (self, other), '*')

        def _backward():
            self.grad += other.data * out.grad
            other.grad += self.data * out.grad
        out._backward = _backward
        return out

# Usage
a = Value(2.0)
b = Value(3.0)
c = a * b
c.grad = 1.0
c._backward()
print(f"dc/da = {a.grad}, dc/db = {b.grad}")  # 3.0, 2.0
</code></pre>"#,
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 3: SPECIALIZED ARCHITECTURES
    // ═══════════════════════════════════════════════════════════════════════════

    // LESSON 6: CNNs
    Lesson {
        id: 6,
        title: "The Pattern Scanner",
        subtitle: "Spatial Structure in Data",
        icon: "🔍",
        why_it_matters: "CNNs revolutionized computer vision. By exploiting spatial structure, they achieve superhuman performance on image tasks with far fewer parameters.",

        intuition: r#"<h3>The Sliding Window</h3>

Instead of looking at an entire image at once, scan a small window across it. At each position, ask: "Is there a horizontal edge here? A vertical edge? A corner?"

This is <strong>convolution</strong>: a small filter slides across the input, producing a map of where that pattern appears.

<h3>Weight Sharing: The Key Insight</h3>

A cat in the top-left corner should be detected the same way as a cat in the bottom-right. Traditional networks would need separate weights for each position. CNNs use the <strong>same filter everywhere</strong>—massive parameter reduction!

<h3>The Hierarchy of Patterns</h3>

<div class="mermaid">
graph LR
    I[Image] --> C1[Conv1:<br/>Edges]
    C1 --> P1[Pool]
    P1 --> C2[Conv2:<br/>Textures]
    C2 --> P2[Pool]
    P2 --> C3[Conv3:<br/>Parts]
    C3 --> FC[Classify]
</div>

• Conv Layer 1: Edges (horizontal, vertical, diagonal)
• Conv Layer 2: Textures (fur, stripes, dots)
• Conv Layer 3: Parts (ears, eyes, paws)
• Conv Layer 4: Objects (cat, dog, car)"#,

        demo_explanation: r#"<strong>🎮 Try This:</strong>
1. Upload or select an image
2. See filters at each layer visualized
3. Watch feature maps highlight detected patterns
4. Hover over a feature map to see which input region created it
5. Draw your own filter and see what it detects"#,

        key_takeaways: &[
            "Convolution = sliding filter pattern matching",
            "Weight sharing = same filter at every position",
            "Pooling = downsampling for translation invariance",
            "Hierarchy: simple to complex features",
            "Far fewer parameters than fully-connected networks",
        ],

        going_deeper: r#"Modern architectures add innovations: <strong>ResNet</strong> (2015) introduced skip connections enabling 100+ layer networks. <strong>EfficientNet</strong> (2019) optimized depth, width, and resolution jointly.

<strong>Vision Transformers</strong> (ViT, 2020) now challenge CNNs by treating images as sequences of patches—same attention mechanism as language models. The boundary between vision and language models is blurring."#,

        math_details: r#"<p>2D Convolution:</p>

$$(I * K)(i,j) = \sum_m \sum_n I(i+m, j+n) \cdot K(m,n)$$

<p>Output size calculation:</p>

$$o = \left\lfloor\frac{i - k + 2p}{s}\right\rfloor + 1$$

<p>Where: $i$=input size, $k$=kernel size, $p$=padding, $s$=stride</p>

<p>Number of parameters in conv layer:</p>

$$\text{params} = k \times k \times c_{\text{in}} \times c_{\text{out}} + c_{\text{out}}$$

<p>Compare to fully-connected: $n_{\text{in}} \times n_{\text{out}}$ (millions more!)</p>"#,

        implementation: r#"<h4>CNN in PyTorch</h4>

<pre><code>import torch.nn as nn

class SimpleCNN(nn.Module):
    def __init__(self):
        super().__init__()
        self.conv1 = nn.Conv2d(1, 32, kernel_size=3, padding=1)
        self.conv2 = nn.Conv2d(32, 64, kernel_size=3, padding=1)
        self.pool = nn.MaxPool2d(2, 2)
        self.fc1 = nn.Linear(64 * 7 * 7, 128)
        self.fc2 = nn.Linear(128, 10)

    def forward(self, x):
        # Input: 1 x 28 x 28
        x = self.pool(torch.relu(self.conv1(x)))  # 32 x 14 x 14
        x = self.pool(torch.relu(self.conv2(x)))  # 64 x 7 x 7
        x = x.view(-1, 64 * 7 * 7)  # Flatten
        x = torch.relu(self.fc1(x))
        x = self.fc2(x)
        return x
</code></pre>"#,
    },

    // LESSON 7: RNNs
    Lesson {
        id: 7,
        title: "Memory in Networks",
        subtitle: "Learning from Sequences",
        icon: "🔁",
        why_it_matters: "Language, music, stock prices—many real-world data sources are sequences where order matters. RNNs introduced the concept of memory to neural networks.",

        intuition: r#"<h3>The Conversation Tracker</h3>

Imagine reading a story word by word. At each word, you update your mental summary of what has happened so far. This running summary is your "hidden state."

RNNs work the same way:
1. Read input at time t
2. Combine with previous hidden state
3. Produce new hidden state
4. Output prediction

The hidden state carries information from the past into the future.

<h3>The Vanishing Problem</h3>

Information degrades as it passes through many timesteps. By step 100, information from step 1 is almost entirely lost—like a game of telephone.

The fix: <strong>LSTM</strong> (Long Short-Term Memory) adds explicit gates that control what to remember and what to forget.

<div class="mermaid">
graph LR
    X1[Word 1] --> H1[State 1]
    H1 --> H2[State 2]
    X2[Word 2] --> H2
    H2 --> H3[State 3]
    X3[Word 3] --> H3
    H3 --> Y[Output]
</div>"#,

        demo_explanation: r#"<strong>🎮 Try This:</strong>
Type text and see predictions. Watch hidden state evolve. See which characters influence current prediction. Compare RNN vs LSTM on long-range dependencies."#,

        key_takeaways: &[
            "RNNs maintain hidden state across timesteps",
            "Same weights applied at every timestep",
            "Vanishing gradients limit memory in simple RNNs",
            "LSTM/GRU use gates to control information flow",
        ],

        going_deeper: r#"RNNs dominated sequence modeling until 2017 when Transformers appeared. Transformers replaced recurrence with attention, enabling parallel processing and better long-range dependencies. However, RNN concepts (state, gates) remain influential in newer architectures like Mamba."#,

        math_details: r#"<p>Simple RNN:</p>

$$h_t = \tanh(W_{hh}h_{t-1} + W_{xh}x_t + b)$$
$$y_t = W_{hy}h_t + b_y$$

<p>LSTM adds forget gate $f$, input gate $i$, and output gate $o$:</p>

$$f_t = \sigma(W_f[h_{t-1}, x_t] + b_f)$$
$$i_t = \sigma(W_i[h_{t-1}, x_t] + b_i)$$
$$\tilde{C}_t = \tanh(W_C[h_{t-1}, x_t] + b_C)$$
$$C_t = f_t \odot C_{t-1} + i_t \odot \tilde{C}_t$$
$$o_t = \sigma(W_o[h_{t-1}, x_t] + b_o)$$
$$h_t = o_t \odot \tanh(C_t)$$"#,

        implementation: r#"<pre><code>import torch.nn as nn

class SimpleRNN(nn.Module):
    def __init__(self, input_size, hidden_size, output_size):
        super().__init__()
        self.hidden_size = hidden_size
        self.rnn = nn.RNN(input_size, hidden_size, batch_first=True)
        self.fc = nn.Linear(hidden_size, output_size)

    def forward(self, x):
        # x: (batch, seq_len, input_size)
        out, hidden = self.rnn(x)
        # out: (batch, seq_len, hidden_size)
        out = self.fc(out[:, -1, :])  # Use last timestep
        return out
</code></pre>"#,
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 4: MODERN DEEP LEARNING
    // ═══════════════════════════════════════════════════════════════════════════

    // LESSON 8: Attention
    Lesson {
        id: 8,
        title: "Focus, Not Memory",
        subtitle: "The Revolution That Changed AI",
        icon: "🎯",
        why_it_matters: "Attention is THE breakthrough behind modern AI. ChatGPT, DALL-E, and virtually every state-of-the-art model uses attention as its core mechanism.",

        intuition: r#"<h3>The Translator's Dilemma</h3>

To translate "The cat sat on the mat" to French:
• "The" → look at "cat" to decide gender (le/la)
• "sat" → look at "cat" to conjugate correctly
• "mat" → look at "on" to choose preposition

Different output words need to <strong>focus on different input words</strong>. This "selective attention" is what the mechanism captures.

<h3>Query, Key, Value: The Library Analogy</h3>

Imagine a library:
• <strong>Query:</strong> "I need books about cats"
• <strong>Keys:</strong> Labels on each shelf ("Animals", "History", "Cooking")
• <strong>Values:</strong> The actual books on each shelf

Attention:
1. Compare query to all keys (how relevant is each shelf?)
2. Weight values by relevance (grab more relevant books)
3. Return weighted combination (your reading stack)

<div class="mermaid">
graph LR
    Q[Query:<br/>What am I<br/>looking for?]
    K[Keys:<br/>What does each<br/>position offer?]
    V[Values:<br/>What content<br/>is there?]
    Q --> S[Score:<br/>Q·K]
    K --> S
    S --> W[Softmax:<br/>Weights]
    W --> O[Output:<br/>Weighted V]
    V --> O
</div>

<h3>Self-Attention: Everyone Talks to Everyone</h3>

In self-attention, each position generates its own query AND serves as a key-value pair for others. Every token can attend to every other token in parallel.

This is why Transformers scale so well—no sequential bottleneck!"#,

        demo_explanation: r#"<strong>🎮 Try This:</strong>
1. Enter a sentence
2. Click any word to see what it attends to
3. See attention weights as line thickness
4. Compare different attention heads
5. Watch how attention patterns change during training"#,

        key_takeaways: &[
            "Attention = selective focus on relevant parts",
            "Query-Key-Value: look up, match, retrieve",
            "Self-attention: every position attends to every other",
            "Enables parallelization (unlike RNNs)",
            "Multi-head attention captures different relationship types",
        ],

        going_deeper: r#"Multi-head attention runs multiple attention patterns in parallel, letting the model focus on different types of relationships simultaneously (syntax, semantics, context).

"Attention is All You Need" (Vaswani et al., 2017) introduced the Transformer architecture, initially for translation, now dominating all of AI. The key insight: you don't need recurrence or convolution—attention alone is sufficient."#,

        math_details: r#"<p>Attention mechanism:</p>

$$\text{Attention}(Q, K, V) = \text{softmax}\left(\frac{QK^T}{\sqrt{d_k}}\right)V$$

<p>The $\sqrt{d_k}$ scaling prevents softmax saturation in high dimensions.</p>

<p>Multi-head attention:</p>

$$\text{head}_i = \text{Attention}(QW_i^Q, KW_i^K, VW_i^V)$$
$$\text{MultiHead}(Q, K, V) = \text{Concat}(\text{head}_1, ..., \text{head}_h)W^O$$

<p>Each head learns different attention patterns using separate weight matrices.</p>"#,

        implementation: r#"<pre><code>import torch
import torch.nn as nn

class SelfAttention(nn.Module):
    def __init__(self, embed_size, heads):
        super().__init__()
        self.embed_size = embed_size
        self.heads = heads
        self.head_dim = embed_size // heads

        self.queries = nn.Linear(self.head_dim, self.head_dim, bias=False)
        self.keys = nn.Linear(self.head_dim, self.head_dim, bias=False)
        self.values = nn.Linear(self.head_dim, self.head_dim, bias=False)
        self.fc_out = nn.Linear(heads * self.head_dim, embed_size)

    def forward(self, values, keys, query, mask=None):
        N = query.shape[0]
        value_len, key_len, query_len = values.shape[1], keys.shape[1], query.shape[1]

        # Split embeddings into multiple heads
        values = values.reshape(N, value_len, self.heads, self.head_dim)
        keys = keys.reshape(N, key_len, self.heads, self.head_dim)
        queries = query.reshape(N, query_len, self.heads, self.head_dim)

        # Compute attention
        energy = torch.einsum("nqhd,nkhd->nhqk", [queries, keys])

        if mask is not None:
            energy = energy.masked_fill(mask == 0, float("-1e20"))

        attention = torch.softmax(energy / (self.embed_size ** (1/2)), dim=3)

        out = torch.einsum("nhql,nlhd->nqhd", [attention, values])
        out = out.reshape(N, query_len, self.heads * self.head_dim)

        out = self.fc_out(out)
        return out
</code></pre>"#,
    },

    // LESSON 9: Transformers
    Lesson {
        id: 9,
        title: "The Architecture of Intelligence",
        subtitle: "How GPT and Friends Actually Work",
        icon: "🏛️",
        why_it_matters: "Transformers are the architecture behind GPT, Claude, BERT, and virtually every modern AI breakthrough. Understanding them is understanding modern AI.",

        intuition: r#"<h3>The Assembly Line</h3>

A Transformer is like a sophisticated assembly line:
1. Raw inputs arrive (words as numbers)
2. Position encoding stamps arrival order
3. Each layer refines the representation
4. Self-attention lets pieces communicate
5. Feed-forward networks process individually
6. Final representation predicts next word

<h3>Encoder vs Decoder</h3>

Original Transformers had both:
• <strong>Encoder:</strong> Reads input, builds understanding (BERT style)
• <strong>Decoder:</strong> Generates output, one token at a time (GPT style)

Modern language models often use decoder-only (GPT) or encoder-only (BERT) architectures.

<div class="mermaid">
graph TB
    T[Tokens] --> E[Embed +<br/>Position]
    E --> A[Self-<br/>Attention]
    A --> N1[Add &<br/>Norm]
    N1 --> F[Feed<br/>Forward]
    F --> N2[Add &<br/>Norm]
    N2 --> O[Prediction]
</div>

<h3>Residual Connections: The Information Highway</h3>

Each layer <strong>adds</strong> to the input rather than replacing it. This "skip connection" lets gradients flow easily through 100+ layers and preserves information from earlier processing."#,

        demo_explanation: r#"<strong>🎮 Try This:</strong>
See token embeddings as vectors. Watch attention patterns form. See layer-by-layer transformation. Generate text and see confidence per token."#,

        key_takeaways: &[
            "Transformer = Attention + Feed-Forward + Residuals",
            "Position encoding adds order information",
            "Stacking layers adds depth of understanding",
            "Residual connections enable very deep networks",
            "Decoder-only (GPT) vs Encoder-only (BERT) vs Encoder-Decoder",
        ],

        going_deeper: r#"<strong>Scaling laws</strong> show that Transformer performance improves predictably with more parameters, data, and compute. This predictability enabled the massive investments in GPT-4 and beyond.

<strong>Emergent capabilities</strong> (abilities that suddenly appear at scale) remain mysterious: arithmetic, translation, coding. These weren't explicitly trained but emerge from scale."#,

        math_details: r#"<p>Transformer block:</p>

$$x' = \text{LayerNorm}(x + \text{MultiHeadAttention}(x))$$
$$\text{out} = \text{LayerNorm}(x' + \text{FFN}(x'))$$

<p>Feed-forward network:</p>

$$\text{FFN}(x) = \max(0, xW_1 + b_1)W_2 + b_2$$

<p>Position encoding (sinusoidal):</p>

$$\text{PE}_{(pos, 2i)} = \sin(pos / 10000^{2i/d})$$
$$\text{PE}_{(pos, 2i+1)} = \cos(pos / 10000^{2i/d})$$"#,

        implementation: r#"<pre><code>class TransformerBlock(nn.Module):
    def __init__(self, embed_size, heads, forward_expansion):
        super().__init__()
        self.attention = SelfAttention(embed_size, heads)
        self.norm1 = nn.LayerNorm(embed_size)
        self.norm2 = nn.LayerNorm(embed_size)

        self.feed_forward = nn.Sequential(
            nn.Linear(embed_size, forward_expansion * embed_size),
            nn.ReLU(),
            nn.Linear(forward_expansion * embed_size, embed_size)
        )

    def forward(self, value, key, query, mask):
        attention = self.attention(value, key, query, mask)
        x = self.norm1(attention + query)
        forward = self.feed_forward(x)
        out = self.norm2(forward + x)
        return out
</code></pre>"#,
    },

    // LESSON 10: Scaling Laws
    Lesson {
        id: 10,
        title: "The Scaling Hypothesis",
        subtitle: "Why Bigger Models Keep Getting Better",
        icon: "📊",
        why_it_matters: "The surprising discovery that scaling up models, data, and compute leads to predictable improvements is the key insight driving modern AI development.",

        intuition: r#"<h3>The Predictable Miracle</h3>

Before 2020, AI research was hit-or-miss. Clever ideas sometimes worked, sometimes didn't. Then came <strong>scaling laws</strong>:

"If you 10x your compute, you will reduce your error by X%"

This turned AI from alchemy into engineering. Companies could predict exactly how much compute they needed for their goals.

<h3>The Three Ingredients</h3>

Scaling requires balance:
1. <strong>Parameters:</strong> Model size (billions of weights)
2. <strong>Data:</strong> Training examples (trillions of tokens)
3. <strong>Compute:</strong> Processing power (thousands of GPUs)

Scaling one without the others wastes resources. <strong>Chinchilla-optimal</strong> training balances all three.

<div class="mermaid">
graph LR
    C[Compute] --> L[Loss]
    D[Data] --> L
    P[Parameters] --> L
    L --> |Power Law| Per[Performance]
</div>"#,

        demo_explanation: r#"<strong>🎮 Try This:</strong>
Adjust compute budget slider. See optimal model size vs data size. Watch predicted loss curve. Mark emergent capability thresholds."#,

        key_takeaways: &[
            "Loss decreases predictably with scale",
            "Compute, data, and parameters must scale together",
            "Emergent abilities appear at unpredictable scales",
            "Scaling laws enabled massive AI investments",
        ],

        going_deeper: r#"<strong>Chinchilla</strong> (2022) showed that most models were over-parameterized and under-trained. Optimal training uses roughly 20 tokens per parameter. This shifted the field toward smaller models trained on more data (Llama, Mistral).

<strong>The scaling ceiling:</strong> We may be running out of high-quality internet text. Future scaling may require synthetic data or multimodal sources."#,

        math_details: r#"<p>Scaling law (Kaplan et al., 2020):</p>

$$L(N, D, C) \approx \left(\frac{N_c}{N}\right)^{\alpha_N} + \left(\frac{D_c}{D}\right)^{\alpha_D} + L_\infty$$

<p>Typically $\alpha_N \approx 0.076$, $\alpha_D \approx 0.095$</p>

<p>Chinchilla optimal:</p>

$$N_{\text{opt}} \propto C^{0.50}, \quad D_{\text{opt}} \propto C^{0.50}$$

<p>Roughly: for every parameter, train on 20 tokens.</p>"#,

        implementation: "",
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // PHASE 5: REINFORCEMENT LEARNING & BEYOND
    // ═══════════════════════════════════════════════════════════════════════════

    // LESSON 11: Reinforcement Learning
    Lesson {
        id: 11,
        title: "Learning from Experience",
        subtitle: "Trial, Error, and Reward",
        icon: "🎮",
        why_it_matters: "RL is how AI learns to play games, control robots, and optimize complex systems. It's fundamentally different from supervised learning—there are no labeled examples.",

        intuition: r#"<h3>The Rat in the Maze</h3>

A rat explores a maze. No one tells it where to go. It tries random directions:
• Dead end? Remember to avoid.
• Cheese? Remember this path!

Over time, the rat learns the optimal route. This is <strong>reinforcement learning</strong>: learning from consequences, not instructions.

<h3>The Credit Assignment Problem (Again)</h3>

You win a game of chess after 40 moves. Which move was brilliant? Which was a mistake? RL must distribute credit across a sequence of actions—much harder than supervised learning.

<h3>Exploration vs Exploitation</h3>

The classic dilemma:
• <strong>Exploit:</strong> Use your best known strategy
• <strong>Explore:</strong> Try something new that might be better

Too much exploitation = stuck at mediocre. Too much exploration = never master anything.

<div class="mermaid">
graph LR
    A[Agent] -->|Action| E[Environment]
    E -->|State| A
    E -->|Reward| A
    A --> P[Policy:<br/>State → Action]
</div>"#,

        demo_explanation: r#"<strong>🎮 Try This:</strong>
Watch agent explore randomly at first. See value function develop (bright = good states). Watch policy arrows point toward goal. Add obstacles and see replanning."#,

        key_takeaways: &[
            "RL learns from rewards, not labels",
            "Policy: mapping from states to actions",
            "Value: expected future reward from each state",
            "Exploration-exploitation tradeoff is fundamental",
            "Credit assignment across time is challenging",
        ],

        going_deeper: r#"<strong>Deep RL</strong> (combining neural networks with RL) achieved superhuman performance in Atari, Go, StarCraft, and robotics. However, it remains sample-inefficient and brittle compared to human learning.

<strong>Model-based RL</strong> and world models are active research areas: learn a model of the environment, then plan within it (like mental simulation)."#,

        math_details: r#"<p>Q-Learning update:</p>

$$Q(s,a) \leftarrow Q(s,a) + \alpha[r + \gamma \max_{a'} Q(s',a') - Q(s,a)]$$

<p>Where: $\alpha$=learning rate, $\gamma$=discount factor (typically 0.99)</p>

<p>Policy gradient:</p>

$$\nabla J(\theta) = \mathbb{E}_\tau\left[\sum_t \nabla_\theta \log \pi_\theta(a_t|s_t) R(\tau)\right]$$

<p>Where $\tau$ is a trajectory and $R(\tau)$ is total reward.</p>"#,

        implementation: r#"<pre><code>import numpy as np

class QLearning:
    def __init__(self, n_states, n_actions, alpha=0.1, gamma=0.99):
        self.Q = np.zeros((n_states, n_actions))
        self.alpha = alpha
        self.gamma = gamma

    def choose_action(self, state, epsilon=0.1):
        if np.random.random() < epsilon:
            return np.random.randint(self.Q.shape[1])  # Explore
        return np.argmax(self.Q[state])  # Exploit

    def update(self, state, action, reward, next_state):
        target = reward + self.gamma * np.max(self.Q[next_state])
        self.Q[state, action] += self.alpha * (target - self.Q[state, action])

# Train on grid world
agent = QLearning(n_states=100, n_actions=4)
for episode in range(1000):
    state = 0
    while state != 99:  # Until reach goal
        action = agent.choose_action(state)
        next_state, reward = env.step(state, action)
        agent.update(state, action, reward, next_state)
        state = next_state
</code></pre>"#,
    },

    // LESSON 12: RLHF
    Lesson {
        id: 12,
        title: "Aligning AI with Humans",
        subtitle: "Teaching Preferences, Not Just Tasks",
        icon: "🤝",
        why_it_matters: "RLHF is how ChatGPT became helpful instead of just predictive. It's the secret sauce of modern AI assistants.",

        intuition: r#"<h3>The Preference Oracle</h3>

Base language models predict the next word. But "likely next word" is not the same as "helpful response." GPT-3 could continue your text in many ways—most unhelpful.

RLHF adds a human layer:
1. Generate many possible responses
2. Humans rank them (A is better than B)
3. Train a reward model to predict human preferences
4. Use RL to maximize the reward model

<h3>The Reward Model</h3>

Instead of humans judging every response (impossible at scale), we train a model to predict what humans would prefer. This "learned reward function" guides the language model.

<div class="mermaid">
graph TB
    LM[Language<br/>Model] --> R1[Response A]
    LM --> R2[Response B]
    R1 --> H[Human:<br/>A > B]
    R2 --> H
    H --> RM[Reward<br/>Model]
    RM --> RL[RL<br/>Training]
    RL --> LM2[Aligned<br/>Model]
</div>

<h3>The RLHF Pipeline</h3>

1. <strong>Pre-training:</strong> Learn language from internet text
2. <strong>Supervised Fine-Tuning:</strong> Learn format from human demonstrations
3. <strong>Reward Modeling:</strong> Learn preferences from comparisons
4. <strong>RL Fine-Tuning:</strong> Optimize against reward model"#,

        demo_explanation: r#"<strong>🎮 Try This:</strong>
See a prompt and two responses. Click to select the better one. Watch reward model update. See how language model output shifts. Observe reward hacking attempts."#,

        key_takeaways: &[
            "RLHF bridges prediction and helpfulness",
            "Reward models learn human preferences at scale",
            "KL penalty prevents reward hacking",
            "Alignment is an ongoing research challenge",
        ],

        going_deeper: r#"RLHF has limitations: <strong>reward hacking</strong> (gaming the reward model), distribution shift, and difficulty capturing nuanced preferences.

Alternatives include:
• <strong>Constitutional AI:</strong> Self-critique against principles
• <strong>DPO:</strong> Direct Preference Optimization (bypasses reward model)
• <strong>Debate:</strong> AI systems arguing different positions"#,

        math_details: r#"<p>Reward modeling loss (Bradley-Terry model):</p>

$$L = -\log \sigma(r_\theta(x, y_w) - r_\theta(x, y_l))$$

<p>Where $y_w$ is the preferred response and $y_l$ is the less preferred.</p>

<p>RLHF objective (PPO-style):</p>

$$\max_\pi \mathbb{E}_{x,y \sim \pi}[r(x,y)] - \beta D_{KL}[\pi || \pi_{\text{ref}}]$$

<p>The KL penalty keeps the model close to the original, preventing reward hacking.</p>"#,

        implementation: "",
    },

    // LESSON 13: Generative AI
    Lesson {
        id: 13,
        title: "Creating, Not Classifying",
        subtitle: "From Prediction to Generation",
        icon: "✨",
        why_it_matters: "Generative AI (images, music, text, code) is the application frontier. Understanding how models create new content reveals both their power and limitations.",

        intuition: r#"<h3>The Noise Reversal</h3>

Diffusion models (DALL-E, Stable Diffusion) work by learning to reverse noise:
1. Take a real image
2. Gradually add noise until it's pure static
3. Train a network to predict and remove the noise
4. At generation: start from noise, repeatedly denoise

The magic: the denoising network learns what "real images" look like in order to remove noise. This understanding enables generation!

<h3>Autoregressive Generation</h3>

Language models generate one token at a time:
1. Given context, predict next token probability
2. Sample a token
3. Add to context
4. Repeat

Simple but powerful. GPT-4 generates entire essays this way, one token at a time.

<div class="mermaid">
graph LR
    N[Pure<br/>Noise] --> D1[Denoise<br/>Step 1]
    D1 --> D2[Denoise<br/>Step 2]
    D2 --> D3[...]
    D3 --> I[Clean<br/>Image]
</div>

<h3>The Latent Space</h3>

Generative models learn compressed representations (latent space) where similar concepts cluster together. Navigating this space enables:
• <strong>Interpolation:</strong> Blend between images
• <strong>Arithmetic:</strong> "King - Man + Woman = Queen"
• <strong>Style transfer:</strong> Apply one image's style to another"#,

        demo_explanation: r#"<strong>🎮 Try This:</strong>
Watch denoising process step by step. Navigate latent space to explore variations. Interpolate between generated examples. See intermediate diffusion steps."#,

        key_takeaways: &[
            "Generative models learn data distributions",
            "Diffusion: reverse gradual corruption",
            "Autoregressive: one token at a time",
            "Latent space captures semantic similarity",
        ],

        going_deeper: r#"<strong>GANs</strong> (Generative Adversarial Networks) use a discriminator to train a generator in an adversarial game. <strong>VAEs</strong> (Variational Autoencoders) learn explicit latent spaces.

Modern systems often combine approaches: Stable Diffusion uses a VAE encoder/decoder with diffusion in latent space (much more efficient than pixel-space diffusion)."#,

        math_details: r#"<p>Diffusion forward process (adding noise):</p>

$$q(x_t|x_{t-1}) = \mathcal{N}(x_t; \sqrt{1-\beta_t}x_{t-1}, \beta_t I)$$

<p>Reverse process (learned denoising):</p>

$$p_\theta(x_{t-1}|x_t) = \mathcal{N}(x_{t-1}; \mu_\theta(x_t, t), \sigma_t^2 I)$$

<p>VAE loss (Evidence Lower Bound):</p>

$$\mathcal{L} = \mathbb{E}_{q(z|x)}[\log p(x|z)] - D_{KL}[q(z|x) || p(z)]$$"#,

        implementation: "",
    },

    // LESSON 14: Multimodal AI
    Lesson {
        id: 14,
        title: "Connecting Senses",
        subtitle: "When AI Sees, Hears, and Speaks",
        icon: "🔗",
        why_it_matters: "The future of AI is multimodal—understanding images, text, audio, and video together. This is how AI becomes truly useful in the real world.",

        intuition: r#"<h3>The Shared Embedding Space</h3>

CLIP (Contrastive Language-Image Pre-training) learns to put images and text in the same mathematical space:
• "A photo of a cat" → vector
• [Image of cat] → nearby vector
• [Image of dog] → farther vector

This shared space enables:
• Image search by text
• Zero-shot classification
• Image-text matching

<h3>Contrastive Learning</h3>

The key insight: learn what goes together.
• <strong>Matching pairs</strong> (image + its caption): push together
• <strong>Non-matching pairs:</strong> push apart

No labels needed! The pairing itself is the supervision.

<div class="mermaid">
graph LR
    I[Image] --> IE[Image<br/>Encoder]
    T[Text] --> TE[Text<br/>Encoder]
    IE --> S[Shared<br/>Embedding<br/>Space]
    TE --> S
    S --> M{Match?}
</div>

<h3>The Vision Language Model</h3>

Modern multimodal models (GPT-4V, Claude) combine:
1. Vision encoder: Extract image features
2. Projection layer: Map to language model space
3. Language model: Reason about image and text together"#,

        demo_explanation: r#"<strong>🎮 Try This:</strong>
See images and text as points in 2D (t-SNE projection). Draw query text, find nearest images. Click image, see nearest text descriptions. Watch training push matches together."#,

        key_takeaways: &[
            "Shared embedding space connects modalities",
            "Contrastive learning: push matches together",
            "Zero-shot transfer: use text to classify images",
            "Multimodal models reason across senses",
        ],

        going_deeper: r#"Vision-language models can now solve complex visual reasoning tasks. Gemini and GPT-4V integrate vision deeply into language models. Audio, video, and other modalities are following the same pattern: encode into a shared space, let the language model reason.

<strong>Future:</strong> Truly unified models that seamlessly handle any combination of text, images, audio, video, and sensors."#,

        math_details: r#"<p>CLIP contrastive loss:</p>

$$L = -\frac{1}{N}\sum_i \log \frac{\exp(\text{sim}(I_i, T_i)/\tau)}{\sum_j \exp(\text{sim}(I_i, T_j)/\tau)}$$

<p>Where $\text{sim}$ is cosine similarity and $\tau$ is temperature parameter.</p>

<p>Symmetric version maximizes both image-to-text and text-to-image matching.</p>"#,

        implementation: "",
    },

    // LESSON 15: AI Safety
    Lesson {
        id: 15,
        title: "The Road Ahead",
        subtitle: "Challenges, Risks, and Possibilities",
        icon: "🛡️",
        why_it_matters: "AI is reshaping society. Understanding its limitations, risks, and potential helps us build systems that benefit humanity.",

        intuition: r#"<h3>The Alignment Problem</h3>

We don't know how to specify exactly what we want. Imagine telling a robot: "Make humans happy."
• <strong>Extreme solution:</strong> Drug everyone
• <strong>Problem:</strong> We meant something more nuanced

AI systems optimize for their objective function. If we specify the wrong objective, they pursue the wrong goal—possibly catastrophically at superhuman capability.

<h3>Capabilities vs Safety</h3>

The field is in a race:
• <strong>Capabilities research:</strong> Make AI more powerful
• <strong>Safety research:</strong> Make AI more controllable

Ideally, safety keeps pace with capabilities. Currently, it's behind.

<div class="mermaid">
graph LR
    C[Capabilities] --> P[Power]
    S[Safety] --> A[Alignment]
    P -->|Outpaces?| D[Danger<br/>Zone]
    A -->|Keeps Up| B[Beneficial<br/>AI]
</div>

<h3>Current Limitations</h3>

What AI still cannot do well:
• Common sense reasoning
• Understanding causation (vs correlation)
• Handling novel situations
• Being reliably truthful
• Knowing what it doesn't know"#,

        demo_explanation: r#"<strong>🎮 Explore Failure Modes:</strong>
1. Adversarial examples: Tiny changes fool vision models
2. Prompt injection: Malicious instructions hidden in input
3. Hallucination: Confident fabrication
4. Specification gaming: Following the letter, not the spirit
5. Distribution shift: Failure on slightly different data"#,

        key_takeaways: &[
            "Alignment: making AI pursue the right goals",
            "Robustness: handling edge cases and attacks",
            "Interpretability: understanding what AI is doing",
            "These are open research problems, not solved",
            "Progress in AI safety is critical for the future",
        ],

        going_deeper: r#"Active research areas include:
• <strong>Mechanistic interpretability:</strong> Reverse-engineering neural networks to understand their internals
• <strong>Constitutional AI:</strong> Models that self-critique against principles
• <strong>Debate:</strong> AI systems arguing different positions for human judgment
• <strong>Scalable oversight:</strong> Humans supervising superhuman AI

<strong>The challenge:</strong> As AI becomes more capable than humans in domains, how do we evaluate its outputs? This is the scalable oversight problem."#,

        math_details: r#"<p>Goodhart's Law (informal):</p>

<em>"When a measure becomes a target, it ceases to be a good measure."</em>

<p>Formally, optimizing a proxy $\hat{U}$ instead of true utility $U$:</p>

$$\arg\max_\pi \hat{U}(\pi) \neq \arg\max_\pi U(\pi)$$

<p>The gap grows with optimization pressure. This is why reward hacking occurs in RLHF.</p>

<p>Mesa-optimization: Systems trained to maximize reward may develop internal optimizers pursuing different goals—a potential source of misalignment.</p>"#,

        implementation: "",
    },
];
