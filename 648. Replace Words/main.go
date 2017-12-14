package main

import (
	"log"
	"strings"
)

// In English, we have a concept called root, which can be followed by some other words to form another longer word - let's call this word successor. For example, the root an, followed by other, which can form another word another.

// Now, given a dictionary consisting of many roots and a sentence. You need to replace all the successor in the sentence with the root forming it. If a successor has many roots can form it, replace it with the root with the shortest length.

// You need to output the sentence after the replacement.

// Example 1:
// Input: dict = ["cat", "bat", "rat"]
// sentence = "the cattle was rattled by the battery"
// Output: "the cat was rat by the bat"
// Note:
// The input will only have lower-case letters.
// 1 <= dict words number <= 1000
// 1 <= sentence words number <= 1000
// 1 <= root length <= 100
// 1 <= sentence words length <= 1000

const (
	maxSliceSize = 26
)

type trieNode struct {
	links    []*trieNode
	isEnding bool
}

func newTrieNode() *trieNode {
	return &trieNode{
		links: make([]*trieNode, maxSliceSize),
	}
}

func (t *trieNode) containsKey(c rune) bool {
	return t.links[c-'a'] != nil
}

func (t *trieNode) get(c rune) (node *trieNode) {
	return t.links[c-'a']
}

func (t *trieNode) put(c rune) {
	t.links[c-'a'] = newTrieNode()
}

func (t *trieNode) setEnd() {
	t.isEnding = true
}

func (t *trieNode) isEnd() bool {
	return t.isEnding
}

type trie struct {
	root *trieNode
}

func newTrie() *trie {
	return &trie{
		root: newTrieNode(),
	}
}

func (t *trie) insert(word string) {
	node := t.root
	for _, c := range word {
		if !node.containsKey(c) {
			node.put(c)
		}
		node = node.get(c)
	}
	node.setEnd()
}

func (t *trie) searchPrefix(word string) *trieNode {
	node := t.root
	for _, c := range word {
		if node.containsKey(c) {
			node = node.get(c)
		} else {
			return nil
		}
	}
	return node
}

func (t *trie) search(word string) bool {
	node := t.searchPrefix(word)
	return node != nil && node.isEnd()
}

func replaceWords(dict []string, sentence string) string {
	trie := newTrie()
	for _, d := range dict {
		trie.insert(d)
	}

	st := strings.Split(sentence, " ")
	for i, s := range st {
		for j := range s {
			w := s[:j+1]
			if trie.search(w) {
				st[i] = w
				break
			}
		}
	}

	return strings.Join(st, " ")
}

func main() {
	testCases := []struct {
		description   string
		inputDict     []string
		inputSentence string
		expect        string
	}{
		{
			description:   "testing 1",
			inputDict:     []string{"cat", "bat", "rat"},
			inputSentence: "the cattle was rattled by the battery",
			expect:        "the cat was rat by the bat",
		},
		{
			description:   "testing 2",
			inputDict:     []string{"a", "aa", "aaa", "aaaa"},
			inputSentence: "a aa a aaaa aaa aaa aaa aaaaaa bbb baba ababa",
			expect:        "a a a a a a a a bbb baba a",
		},
	}

	for _, testCase := range testCases {
		actual := replaceWords(testCase.inputDict, testCase.inputSentence)
		if testCase.expect != actual {
			log.Fatalf("%s: expect[%s] != actual[%s]", testCase.description, testCase.expect, actual)
		}
	}
}
