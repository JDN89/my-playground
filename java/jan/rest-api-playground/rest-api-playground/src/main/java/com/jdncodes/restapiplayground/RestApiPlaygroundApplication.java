package com.jdncodes.restapiplayground;

import com.jdncodes.restapiplayground.entity.SecretConfigProperties;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.boot.context.properties.EnableConfigurationProperties;

@SpringBootApplication
@EnableConfigurationProperties(SecretConfigProperties.class)
public class RestApiPlaygroundApplication {

	public static void main(String[] args) {
		SpringApplication.run(RestApiPlaygroundApplication.class, args);
	}

}
