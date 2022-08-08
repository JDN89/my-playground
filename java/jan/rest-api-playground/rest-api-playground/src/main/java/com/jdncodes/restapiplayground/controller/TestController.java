package com.jdncodes.restapiplayground.controller;

import com.jdncodes.restapiplayground.entity.SecretConfigProperties;
import com.jdncodes.restapiplayground.services.DepartmentService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.Map;

@RestController
@RequiredArgsConstructor
public class TestController {
    private final DepartmentService departmentService;
    private final SecretConfigProperties secret;
    @GetMapping("/hello")
    public Map<String,String> printSecrets() {
        return Map.of("apiUrl", secret.apiUrl(),
                "ApiVersion", secret.apiVersion(),
                "password", secret.password());
    }
}
