#include <cstddef>
#include <unordered_map>
#include <unordered_set>
#include <vector>
using namespace std;

class Node {
public:
  int val;
  vector<Node *> neighbors;
  Node() {
    val = 0;
    neighbors = vector<Node *>();
  }
  Node(int _val) {
    val = _val;
    neighbors = vector<Node *>();
  }
  Node(int _val, vector<Node *> _neighbors) {
    val = _val;
    neighbors = _neighbors;
  }
};

class Solution {
public:
  Node *cloneGraph(Node *node) {
    if (!node) {
      return NULL;
    }
    Node *start = node;
    unordered_map<Node *, Node> counter_map;
    vector<Node *> stk({start});
    unordered_set<Node *> visited;

    while (!stk.empty()) {
      Node *nd = stk.back();
      stk.pop_back();
      counter_map[nd] = Node(nd->val);

      for (auto x : nd->neighbors) {
        if (visited.find(x) != visited.end()) {
          visited.insert(x);
          stk.push_back(x);
        }
      }
    }
    for (auto it : counter_map) {
      for (auto x : it.first->neighbors) {
        Node new_niegh = counter_map[x];
        it.second.neighbors.push_back(&new_niegh);
      }
    }
    return &counter_map[start];
  }
};
