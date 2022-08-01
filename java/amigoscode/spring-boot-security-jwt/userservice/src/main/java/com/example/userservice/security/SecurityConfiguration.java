package com.example.userservice.security;

import lombok.RequiredArgsConstructor;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.security.config.annotation.web.configuration.EnableWebSecurity;
import org.springframework.security.core.userdetails.User;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.security.core.userdetails.UserDetailsService;
import org.springframework.security.crypto.bcrypt.BCryptPasswordEncoder;
import org.springframework.security.provisioning.InMemoryUserDetailsManager;
// WebSecurityConfigurerAdapter is deprecated
// explanation of the new way of doing things
// https://spring.io/blog/2022/02/21/spring-security-without-the-websecurityconfigureradapter
@Configuration @EnableWebSecurity @RequiredArgsConstructor
    public class SecurityConfiguration {

private final UserDetailsService userDetailsService;
private final BCryptPasswordEncoder bCryptPasswordEncoder;

    @Bean
    public InMemoryUserDetailsManager userDetailsService() {
        auth
        UserDetails user = User.withDefaultPasswordEncoder()
                .username("user")
                .password("password")
                .roles("USER")
                .build();
        return new InMemoryUserDetailsManager(user);
    }
    }


