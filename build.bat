cd PermissionLib
cargo build
cargo test
cd ..

copy PermissionLib\bindings\csharp\PermissionLib.cs PermissionApp\
cd PermissionApp
dotnet build
cd ..

copy PermissionLib\target\debug\permission_lib.dll PermissionApp\bin\Debug\net8.0\
.\PermissionApp\bin\Debug\net8.0\PermissionApp.exe
