package com.jdn.restdemo.entity;

import org.springframework.boot.context.properties.ConfigurationProperties;


@ConfigurationProperties("secret")
public record SecretConfigProperties(String apiUrl, String apiVersion, String password) {
}
