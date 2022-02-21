#include <iostream>
#include "crash.h"

using namespace std;

int main() {
    cout << "calling create_db" << endl;
    auto db = ns::create_db("path");
    cout << "create_db returned" << endl;

    cout << "calling nocrash" << endl;
    cout << db->nocrash() << endl;
    cout << "nocrash returned" << endl;

    cout << "calling crash" << endl;
    cout << db->crash() << endl;
    // below line was not printed on windows
    cout << "crash returned" << endl;

    return 0;
}
