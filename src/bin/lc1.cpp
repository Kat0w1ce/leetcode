#include <assert.h>
#include <map>
#include <vector>
using std::map;
using std::vector;
vector<int> twoSum(vector<int> &nums, int target);
int main(int argc, char const *argv[]) {
    /* code */
    auto v = vector<int>{3, 3};
    auto target = 6;
    auto rst = twoSum(v, target);
    assert(rst[0] == 0);
    assert(rst[1] == 1);
    return 0;
}

vector<int> twoSum(vector<int> &nums, int target) {
    auto rst = vector<int>{};
    map<unsigned int, int> hm;
    for (auto i = 0; i < nums.size(); i++) {
        int left = target - nums[i];
        auto fd = hm.find(left);
        if (fd != hm.end()) {
            rst.push_back(hm[left]);
            rst.push_back(i);
            break;
        } else {
            hm[nums[i]] = i;
        }
    }
    return rst;
}