$file = "c:\Users\Davi\Desktop\codes\Snask\snask_compiler\src\semantic_analyzer.rs"
$content = Get-Content $file -Raw -Encoding UTF8

# Encontrar a linha onde adicionar as novas funções (antes do fechamento da função register_stdlib)
$searchText = '        self.define_builtin("arch", vec![], Type::String);'
$newFunctions = @"
        self.define_builtin("arch", vec![], Type::String);

        // Math - Novas funções
        self.define_builtin("mod", vec![Type::Float, Type::Float], Type::Float);
        self.define_builtin("random", vec![], Type::Float);
        self.define_builtin("random_range", vec![Type::Float, Type::Float], Type::Float);
        self.define_builtin("clamp", vec![Type::Float, Type::Float, Type::Float], Type::Float);
        self.define_builtin("sign", vec![Type::Float], Type::Float);
        self.define_builtin("deg_to_rad", vec![Type::Float], Type::Float);
        self.define_builtin("rad_to_deg", vec![Type::Float], Type::Float);
"@

$content = $content -replace [regex]::Escape($searchText), $newFunctions

Set-Content -Path $file -Value $content -Encoding UTF8 -NoNewline

Write-Host "Arquivo atualizado com sucesso!"
