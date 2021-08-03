#include <assert.h>
#include <iostream>
#include <map>
#include <string>
#include <vector>
using std::map;
using std::string;
using namespace std;

int lengthOfLongestSubstring(string s);
int main() {
    assert(lengthOfLongestSubstring("abcabcbb") == 3);
    assert(lengthOfLongestSubstring("bbbbb") == 1);
    assert(lengthOfLongestSubstring("pwwkew") == 3);
    // if (lengthOfLongestSubstring("abcabcbb") == 3)
    //     cout<<
    // cout << lengthOfLongestSubstring("abcabcbb") << endl;
    return 0;
}

int lengthOfLongestSubstring(string s) {
    // map<char, int> m;
    // 可用vector优化;
    vector<int> m = vector<int>(256, 0);
    int rst = 0, len = 0;
    int sbegin = 0, send = 0;
    while (send < s.size()) {
        if (m[s[send]] == 0) {
            m[s[send++]] = 1;
            len++;
        } else if (m[s[send]] == 1) {
            if (len > rst)
                rst = len;
            m[s[sbegin]]--;
            sbegin++;
            len--;
        }
    }

    return len > rst ? len : rst;
}
// aab
// abcbcda
// abbca