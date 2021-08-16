#include <string>
using std::string;
int lengthOfLastWord(string s) {
  int len = 0;
  auto flag = false;

  for (auto i = s.rbegin(); i != s.rend(); i++) {
    if (*i != ' ' && flag == true) {
      len++;
    } else if (*i != ' ' && flag == false) {
      len++;
      flag = true;
    } else if (*i == ' ' && flag == true) {
      break;
    } else {
      continue;
    }
  }
  return len;
}