cd PermissionApp
dotnet build

cd ..
cd PermissionLib
cargo build

cd ..

copy PermissionLib\target\debug\permission_lib.dll PermissionApp\bin\Debug\net8.0\
.\PermissionApp\bin\Debug\net8.0\PermissionApp.exe