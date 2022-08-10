package com.jdn.restdemo;

import com.github.javafaker.Faker;
import com.jdn.restdemo.entity.SecretConfigProperties;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.boot.context.properties.EnableConfigurationProperties;
import org.springframework.context.annotation.Bean;

@SpringBootApplication
@EnableConfigurationProperties(SecretConfigProperties.class)
public class RestdemoApplication {

	public static void main(String[] args) {
		SpringApplication.run(RestdemoApplication.class, args);
	}

	@Bean
	public Faker faker() {
		return new Faker();
	}

}
