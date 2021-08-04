#include <assert.h>
#include <math.h>
#include <string>
int reverse(int x);
int main() {
    assert(reverse(120) == 21);
    return 0;
}
int reverse(int x) {
    int res = 0;
    int xx = abs(x);
    while (xx != 0) {
        if (abs(res) > INT32_MAX / 10)
            return 0;
        res = res * 10 + xx % 10;
        xx /= 10;
        /* code */
    }
    return x > 0 ? res : -res;
}