package com.jdn.restdemo.controller;

import com.jdn.restdemo.entity.SecretConfigProperties;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.Map;

@RestController
@RequestMapping("/test")
@RequiredArgsConstructor
public class SecretsController {
    private final SecretConfigProperties secret;

    @GetMapping("/hello")
    public Map<String,String> printSecrets() {
        return Map.of("apiUrl: ", secret.apiUrl(),
                "Api Version: " ,secret.apiVersion(),
                "password: ", secret.password());
    }
}
