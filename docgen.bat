@echo off
setlocal
chcp 65001 >nul  :: コードページをUTF-8に変更

if "%~1"=="" (
    echo エラー: ファイルやディレクトリが渡されていません。
    exit /b 1
)

:: すべてのドラッグ・アンド・ドロップ対象を逐次処理
:loop
if "%~1"=="" (
    goto endloop
)

set "basepath=%~1"



:: コマンド実行
echo "実行中: docgen --dir \"%basepath%\""
docgen -d "%basepath%"
if %errorlevel% neq 0 (
    echo エラー: docgenの実行に失敗しました。
    exit /b 1
)

pause

:endloop
endlocal